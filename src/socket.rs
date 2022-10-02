use crate::config::Config;
use crate::geocoding::Geocoding;
use crate::logger::LOGGER;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

pub async fn listener(config: Config, geocoding: &Geocoding) {
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
                process(socket, geocoding).await;
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

async fn process(_socket: TcpStream, _geocoding: &Geocoding) {
    // Todo process the command
}
