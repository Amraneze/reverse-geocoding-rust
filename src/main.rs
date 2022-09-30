use reverse_geocoding::cache::Cache;
use reverse_geocoding::logger::LOGGER;
use std::env;
use std::io::Result;

#[macro_use]
extern crate slog;

static mut CACHE: Option<Cache> = None;

fn parse_file_name(args: &Vec<String>) -> String {
    if args.len() < 2 {
        panic!("File path is not provided as argument. Use the format cargo ... -- file_path");
    }
    let file_name: String = args[1].clone();
    debug!(LOGGER, "File name used is {}", file_name);
    return file_name;
}

pub fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = parse_file_name(&args);
    unsafe {
        CACHE = Some(Cache::parse_buffer(&file_name)?);
        lookup();
    }
    Ok(())
}

pub unsafe fn lookup() -> Result<()> {
    match &mut CACHE {
        Some(cache) => {
            let result = cache.lookup(-73.9865812, 40.7305991);
            println!("result {:?}", result);
        },
        None => panic!("Geocoding Cache is not available"),
    }
    Ok(())
}
