use reverse_geocoding::geocoding::Geocoding;
use std::env;
use std::io::Result;

pub fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut _geocoding: Geocoding = Geocoding::new(&args);
    Ok(())
}
