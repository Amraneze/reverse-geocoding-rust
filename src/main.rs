use reverse_geocoding::config::Config;
use reverse_geocoding::geocoding::Geocoding;
use reverse_geocoding::socket::listener;
use std::io::Result;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    let options = Config::from_args();
    let geocoding: Arc<Mutex<Geocoding>> = Arc::new(Mutex::new(Geocoding::new(&options)));
    listener(options, &geocoding).await;
    Ok(())
}
