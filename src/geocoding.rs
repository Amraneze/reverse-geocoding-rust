use crate::cache::Cache;
use crate::config::Config;
use crate::logger::LOGGER;

#[derive(Debug, Default, Clone)]
pub struct Geocoding {
    cache: Cache,
}

impl Geocoding {
    pub fn new(options: &Config) -> Self {
        let cache: Cache = Cache::parse_buffer(&options.file_path)
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
