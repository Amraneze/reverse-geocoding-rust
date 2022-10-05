use structopt::StructOpt;

#[derive(Debug, StructOpt, Default)]
#[structopt(
    name = "reverse-geocoding-client",
    about = "A client for the memory cache app for reverse geocoding"
)]
pub struct Config {
    // Reverse Memory Cache host
    #[structopt(short = "h", long = "host", default_value = "127.0.0.1")]
    pub host: String,
    // Socket port to be used for communication
    #[structopt(short = "p", long = "port", default_value = "4020")]
    pub port: u64,
    // Latitude
    #[structopt(short = "t", long = "latitude")]
    pub latitude: f32,
    // Longitude
    #[structopt(short = "g", long = "longitude")]
    pub longitude: f32,
}
