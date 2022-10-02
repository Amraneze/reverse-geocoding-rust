use structopt::StructOpt;

#[derive(Debug, StructOpt, Default)]
#[structopt(name = "reverse-geocoding", about = "A memory cache reverse geocoding")]
pub struct Config {
    // OSM binary path file
    #[structopt(short = "f", long = "path-file")]
    pub file_path: String,
    // Socket port to be used for communication
    #[structopt(short = "p", long = "port", default_value = "4020")]
    pub port: u64,
}
