use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use std::error::Error;
use std::str::from_utf8;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("127.0.0.1:4020").await?;
    let (mut read_stream, mut write_stream) = io::split(stream);
    println!("Connection established");
    tokio::spawn(async move {
        println!("Sending data");
        match write_stream.write(b"LOOKUP 2.320041 48.8588897").await {
            Ok(_) => (),
            Err(error) => panic!(
                "An issue occurred while writing command with error {}",
                error
            ),
        }
    });
    let mut buffer = vec![0; 1024];
    loop {
        match read_stream.read(&mut buffer).await {
            Err(error) => panic!(
                "Couldn't read the end of the command sent via sockets {:?}",
                error
            ),
            // Socket closed
            Ok(length) if length == 0 => break,
            Ok(length) => {
                match from_utf8(&buffer[..length]) {
                    Ok(parsed_result) => println!("{}", parsed_result),
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
