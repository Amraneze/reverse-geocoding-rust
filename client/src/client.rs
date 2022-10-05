use reverse_geocoding_client::config::Config;
use structopt::StructOpt;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;

use std::error::Error;
use std::str::from_utf8;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let options = Config::from_args();
    let address = format!("{}:{}", options.host, options.port);
    let stream = TcpStream::connect(address).await?;
    let (mut read_stream, mut write_stream): (ReadHalf<TcpStream>, WriteHalf<TcpStream>) =
        io::split(stream);
    println!("Connection established");
    tokio::spawn(async move {
        println!("Sending data");
        let latitude = options.latitude;
        let longitude = options.longitude;
        let command = format!("LOOKUP {} {}", longitude, latitude);
        match write_stream.write(&command.as_bytes().to_vec()).await {
            Ok(_) => println!("Command was sent successfully"),
            Err(error) => panic!(
                "An issue occurred while writing command with error {}",
                error
            ),
        }
    });
    let mut buffer = vec![0; 1024];
    loop {
        println!("Reading command's result");
        match read_stream.read_buf(&mut buffer).await {
            Err(error) => panic!(
                "Couldn't read the end of the command sent via sockets {:?}",
                error
            ),
            // Socket closed
            Ok(length) if length == 0 => break,
            Ok(length) => {
                match from_utf8(&buffer[..length]) {
                    Ok(parsed_result) => println!("Result: {}", parsed_result),
                    Err(error) => panic!(
                        "An error occurred while parsing result with encoding UTF-8: {}",
                        error
                    ),
                };
                break;
            }
        }
    }
    Ok(())
}
