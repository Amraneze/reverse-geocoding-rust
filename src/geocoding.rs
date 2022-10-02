use crate::cache::Cache;
use crate::logger::LOGGER;

fn parse_file_name(args: &Vec<String>) -> String {
    if args.len() < 2 {
        panic!("File path is not provided as argument. Use the format cargo ... -- file_path");
    }
    let file_name: String = args[1].clone();
    debug!(LOGGER, "File name used is {}", file_name);
    return file_name;
}

#[derive(Debug, Default)]
pub struct Geocoding {
    cache: Cache,
}

impl Geocoding {
    pub fn new(args: &Vec<String>) -> Self {
        let file_name: String = parse_file_name(&args);
        let cache: Cache = Cache::parse_buffer(&file_name)
            .expect("An error occurred while generating Geocode reverse cache");
        return Self { cache };
    }

    pub fn lookup(&mut self, longitude: f32, latitude: f32) -> Vec<String> {
        debug!(LOGGER, "longitude: {}, latitude: {}", longitude, latitude);
        return self.cache.lookup(longitude, latitude);
    }
}

#[path = "../tests/geocoding/geocoding.rs"]
#[cfg(test)]
mod tests;
