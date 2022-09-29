use reverse_geocoding::logger::LOGGER;
use reverse_geocoding::number::ReadBytes;
use std::collections::HashMap;
use std::env;
use std::fs::read;
use std::io::{Cursor, Read, Result};

#[macro_use]
extern crate slog;
extern crate hex;

static mut CACHE: Option<HashMap<String, String>> = None;

fn parse_file_name(args: &Vec<String>) -> String {
    let file_name: String = args[1].clone();
    debug!(LOGGER, "File name used is {}", file_name);
    return file_name;
}

fn get_file_as_byte_vec(filename: &String) -> Result<Vec<u8>> {
    match read(filename) {
        Ok(bytes) => Ok(bytes),
        Err(e) => panic!(
            "An error occurred while reading file {} with exception {}",
            filename, e
        ),
    }
}

pub fn get_hex_rep(byte_array: &mut [u8]) -> String {
    let build_string_vec: Vec<String> = byte_array
        .chunks(2)
        .map(|c| {
            if c.len() == 2 {
                format!("{:02x}{:02x}", c[0], c[1])
            } else {
                format!("{:02x}", c[0])
            }
        })
        .collect();

    build_string_vec.join(" ")
}

pub fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = parse_file_name(&args);
    let mut buffer = Cursor::new(get_file_as_byte_vec(&file_name).unwrap());
    let magic_number = <dyn Read as ReadBytes>::read::<u32>(&mut buffer);
    if magic_number != 1845944321 {
        panic!("Index file does not have the correct type or version.")
    }
    let width = <dyn Read as ReadBytes>::read::<u32>(&mut buffer);
    let height = <dyn Read as ReadBytes>::read::<u32>(&mut buffer);
    let x_scale = <dyn Read as ReadBytes>::read::<f32>(&mut buffer);
    let y_scale = <dyn Read as ReadBytes>::read::<f32>(&mut buffer);
    let x_shift = <dyn Read as ReadBytes>::read::<u32>(&mut buffer);
    let y_shift = <dyn Read as ReadBytes>::read::<u32>(&mut buffer);
    let numentries = <dyn Read as ReadBytes>::read::<u32>(&mut buffer);
    debug!(
        LOGGER,
        "Width: {}, Height: {}, X_Scale: {}, Y_Scale: {}, X_Shift: {}, Y_Shift: {}, numentries: {}",
        width,
        height,
        x_scale,
        y_scale,
        x_shift,
        y_shift,
        numentries
    );
    unsafe {
        CACHE = Some(HashMap::with_capacity(numentries as usize));
        match &CACHE {
            Some(cache) => debug!(LOGGER, "Cache size: {}", cache.capacity(),),
            None => panic!("Cache can't be empty"),
        }
    }
    Ok(())
}
