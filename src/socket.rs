use crate::commands::{Commands, FromString};
use crate::config::Config;
use crate::geocoding::Geocoding;
use crate::logger::LOGGER;
use crate::types::Command;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use std::str;
use std::sync::{Arc, Mutex};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};

pub async fn listener(config: Config, shared_geocoding: &Arc<Mutex<Geocoding>>) {
    let address_str = format!("{}:{}", "127.0.0.1", config.port);
    let address = parse_address(&address_str);
    let listener = bind_address(&config, &address).await;

    info!(LOGGER, "Start listening on: {}", address);
    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                debug!(LOGGER, "Connection accepted");
                process(socket, shared_geocoding).await;
            }
            Err(error) => {
                panic!("{:?}", Error::new(
                    ErrorKind::ConnectionRefused,
                    format!(
                        "Reverse Geocoding couldn't start listening to the address {} with error: {:?}",
                        address_str, error
                    ),
                ));
            }
        };
    }
}

fn parse_address(address_str: &String) -> SocketAddr {
    return match address_str.parse::<SocketAddr>() {
        Ok(socket_address) => socket_address,
        Err(error) => {
            panic!(
                "{:?}",
                Error::new(
                    ErrorKind::ConnectionRefused,
                    format!(
                        "Reverse Geocoding couldn't start listening to the address {} with error: {:?}",
                        address_str, error
                    ),
                )
            );
        }
    };
}

async fn bind_address(config: &Config, address: &SocketAddr) -> TcpListener {
    return match TcpListener::bind(address).await {
        Ok(socket) => socket,
        Err(error) => {
            if config.port <= 1024 {
                warn!(
                    LOGGER,
                    "You are using a privileged port: {}. You should use a port > 1024",
                    config.port
                );
            }
            panic!(
                "{:?}",
                Error::new(
                    ErrorKind::ConnectionRefused,
                    format!(
                        "Reverse Geocoding couldn't start listening to the address {:?} with error: {:?}",
                        address, error
                    ),
                )
            );
        }
    };
}

async fn process(stream: TcpStream, shared_geocoding: &Arc<Mutex<Geocoding>>) {
    let (mut read_stream, mut write_stream): (ReadHalf<TcpStream>, WriteHalf<TcpStream>) =
        io::split(stream);
    let geocoding = Arc::clone(shared_geocoding);
    tokio::spawn(async move {
        let mut buffer = vec![0; 1024];
        loop {
            match read_stream.read(&mut buffer).await {
                Err(error) => panic!("Couldn't read the command sent via sockets {:?}", error),
                // Socket closed
                Ok(length) if length == 0 => {
                    println!("Here");
                    return;
                }
                Ok(length) => {
                    process_command(&mut write_stream, &buffer[..length], &geocoding).await
                }
            };
        }
    });
}

async fn process_command(
    write_stream: &mut WriteHalf<TcpStream>,
    buffer: &[u8],
    geocoding: &Arc<Mutex<Geocoding>>,
) {
    let command_with_args = match str::from_utf8(buffer) {
        Ok(parsed_command) => parsed_command,
        Err(error) => panic!(
            "An error occurred while parsing string with encoding UTF-8: {}",
            error
        ),
    };
    match Command::from_str(command_with_args) {
        Ok(command) => match command {
            Commands::Lookup(longitude, latitude) => {
                let geocoding_result = geocoding.lock().unwrap().lookup(longitude, latitude);
                let geocoding_result_buffer = if geocoding_result.is_empty() {
                    vec![0u8; 1]
                } else {
                    geocoding_result.join("|").as_bytes().to_vec()
                };
                debug!(LOGGER, "The result is {:?}", geocoding_result);
                if let Err(error) = write_stream.write(&geocoding_result_buffer).await {
                    panic!(
                        "An error occurred while writing result to the socket: {}",
                        error
                    );
                }
            }
        },
        Err(error) => panic!("{}", String::from(error)),
    }
}
