use crate::commands::{Commands, FromString};
use crate::config::Config;
use crate::geocoding::Geocoding;
use crate::logger::LOGGER;
use crate::types::Command;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use std::str;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, WriteHalf};
use tokio::net::{TcpListener, TcpStream};

pub async fn listener(config: Config, geocoding: Geocoding) {
    let address_str = format!("{}:{}", "127.0.0.1", config.port);
    let address = match address_str.parse::<SocketAddr>() {
        Ok(socket_address) => socket_address,
        Err(error) => {
            error!(
                LOGGER,
                "Reverse Geocoding couldn't start listening to the address {} with error: {}",
                address_str,
                error
            );
            Error::new(
                ErrorKind::ConnectionRefused,
                format!(
                    "Reverse Geocoding couldn't start listening to the address {} with error: {:?}",
                    address_str, error
                ),
            );
            return;
        }
    };

    let listener = match TcpListener::bind(&address).await {
        Ok(socket) => socket,
        Err(error) => {
            error!(
                LOGGER,
                "Reverse Geocoding couldn't start listening to the address {} with error: {:?}",
                address_str,
                error
            );
            if config.port <= 1024 {
                warn!(
                    LOGGER,
                    "You are using a privileged port: {}. You should use a port > 1024",
                    config.port
                );
            }
            Error::new(
                ErrorKind::ConnectionRefused,
                format!(
                    "Reverse Geocoding couldn't start listening to the address {} with error: {:?}",
                    address_str, error
                ),
            );
            return;
        }
    };

    info!(LOGGER, "Start listening on: {}", address);
    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                debug!(LOGGER, "Connection accepted");
                process(socket, &geocoding).await;
            }
            Err(error) => {
                error!(
                    LOGGER,
                    "Failed to connect to: {} with given error: {:?}", address, error
                );
                Error::new(
                    ErrorKind::ConnectionRefused,
                    format!(
                        "Reverse Geocoding couldn't start listening to the address {} with error: {:?}",
                        address_str, error
                    ),
                );
            }
        };
    }
}

async fn process(stream: TcpStream, geocoding_ref: &Geocoding) {
    let mut geocoding = geocoding_ref.clone();
    let (mut read_stream, mut write_stream) = io::split(stream);
    tokio::spawn(async move {
        let mut buffer = vec![0; 1024];
        loop {
            match read_stream.read(&mut buffer).await {
                Err(error) => panic!(
                    "Couldn't read the end of the command sent via sockets {:?}",
                    error
                ),
                // Socket closed
                Ok(length) if length == 0 => return,
                Ok(length) => {
                    process_command(&mut write_stream, &buffer[..length], &mut geocoding).await
                }
            };
        }
    });
}

async fn process_command(
    write_stream: &mut WriteHalf<TcpStream>,
    buffer: &[u8],
    geocoding: &mut Geocoding,
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
                let geocoding_result = geocoding.lookup(longitude, latitude);
                let geocoding_result_buffer = geocoding_result.join("|").as_bytes().to_vec();
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
