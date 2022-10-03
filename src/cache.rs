use crate::io::io;
use crate::logger::LOGGER;
use crate::number::{number, ReadBytes};
use crate::types::Buffer;
use crate::types::Data;
use math::round;
use std::collections::HashMap;
use std::io::{Cursor, Read, Result, Seek};
use std::io::{Error, ErrorKind};

#[derive(Debug, Default, Clone)]
pub struct GeoData {
    width: u32,
    height: u32,
    x_scale: f32,
    y_scale: f32,
    x_shift: f32,
    y_shift: f32,
    numentries: u32,
}

#[derive(Debug, Default, Clone)]
pub struct Cache {
    cache: Data,
    buffer: Buffer,
    geo_data: GeoData,
}

const HEADER_SIZE: u32 = 32;
const MAGIC_NUMBER: u32 = 1845944321;

impl Cache {
    pub fn lookup(&mut self, longitude: f32, latitude: f32) -> Vec<String> {
        let index = self.lookup_uncached(longitude, latitude);
        return match self.lookup_entry(index) {
            Some(address) => address.to_vec(),
            None => {
                debug!(
                    LOGGER,
                    "Couldn't find any geospatial data for longitude: {} & latitude: {}",
                    longitude,
                    latitude
                );
                return vec![];
            }
        };
    }

    fn read_unsigned_i32(&mut self) -> Result<i64> {
        let mut value = 0 as i64;
        let mut bits = 0 as u32;
        loop {
            let mut bit_buffer = vec![0u8; 1];
            self.buffer
                .read_exact(&mut bit_buffer)
                .expect("An error occurred while reading bytes from the buffer");
            let data = i8::from_be_bytes(bit_buffer.try_into().unwrap()) as i64;
            value |= (data & 0x7F) << bits;
            if (data & 0x80) == 0 {
                return Ok(value);
            }
            bits += 7;
            if bits > 35 {
                Error::new(
                    ErrorKind::InvalidData,
                    "Variable length quantity is too long for expected integer",
                );
            }
        }
    }

    fn lookup_uncached(&mut self, longitude: f32, latitude: f32) -> i64 {
        let x = round::floor(
            ((longitude + self.geo_data.x_shift) * self.geo_data.x_scale).into(),
            0,
        ) as i32;
        let y = round::floor(
            ((latitude + self.geo_data.y_shift) * self.geo_data.y_scale).into(),
            0,
        ) as i32;
        if x < 0
            || number::parse_u32(x) >= self.geo_data.width
            || y < 0
            || number::parse_u32(y) >= self.geo_data.height
        {
            return 0;
        }
        let position = (HEADER_SIZE + (number::parse_u32(y) << 2)) as u64;
        self.buffer.set_position(position);
        let row_position = <dyn Read as ReadBytes>::read::<u32>(&mut self.buffer) as u64;
        self.buffer.set_position(row_position);
        let mut i = 0 as i64;
        let x_i64 = i64::from(x);
        while i <= x_i64 {
            let count = self.read_unsigned_i32().unwrap();
            i += self.read_unsigned_i32().unwrap() + 1;
            if x_i64 < i {
                return count;
            }
        }
        return 0;
    }

    fn lookup_entry_uncached(&mut self, index: u32) -> Vec<String> {
        let position = (HEADER_SIZE + ((self.geo_data.height + index) << 2)) as u64;
        self.buffer.set_position(position);
        let start = <dyn Read as ReadBytes>::read::<u32>(&mut self.buffer);
        let end = <dyn Read as ReadBytes>::read::<u32>(&mut self.buffer);
        if start == end {
            return vec![];
        }
        let length = (end - start) as usize;
        self.buffer.set_position(start as u64);
        let mut decoded_buffer = vec![0u8; length];
        self.buffer.read_exact(&mut decoded_buffer).expect(
            format!(
                "An error thrown when reading exactly {} from the buffer",
                length
            )
            .as_str(),
        );
        self.buffer
            .rewind()
            .expect("An error occurred while rewinding the buffer");
        return match std::str::from_utf8(&mut decoded_buffer) {
            Ok(address) => {
                let mut result: Vec<String> = address
                    .split('\0')
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                match result.last() {
                    Some(maybe_empty_string) => {
                        if maybe_empty_string == &"" {
                            // remove the last element because it's an empty string
                            result.pop();
                        }
                    }
                    None => (),
                }
                result
            }
            Err(_) => {
                error!(
                    LOGGER,
                    "We couldn't find any geospatial address for the index {}", index
                );
                vec![]
            }
        };
    }

    fn lookup_entry(&mut self, key: i64) -> Option<&mut Vec<String>> {
        let index = key as u32;
        //let index = number::parse_u32(key as u32);
        if key < 0 || index >= self.geo_data.numentries {
            return None;
        }
        return match self.cache.contains_key(&index) {
            true => self.cache.get_mut(&index),
            false => {
                let cached_entry = self.lookup_entry_uncached(index);
                self.cache.insert(index, cached_entry).as_mut();
                // I can't return the result of insert, I'm getting
                // "cannot return reference to temporary value" error
                return self.cache.get_mut(&index);
            }
        };
    }

    pub fn parse_buffer(file_name: &String) -> Result<Self> {
        let mut buffer = Cursor::new(io::get_file_as_byte_vec(&file_name).unwrap());
        let magic_number = <dyn Read as ReadBytes>::read::<u32>(&mut buffer);
        if magic_number != MAGIC_NUMBER {
            panic!("Index file does not have the correct type or version.");
        }
        let width = <dyn Read as ReadBytes>::read::<u32>(&mut buffer);
        let height = <dyn Read as ReadBytes>::read::<u32>(&mut buffer);
        let x_scale = width as f32 / <dyn Read as ReadBytes>::read::<f32>(&mut buffer);
        let y_scale = height as f32 / <dyn Read as ReadBytes>::read::<f32>(&mut buffer);
        let x_shift = <dyn Read as ReadBytes>::read::<f32>(&mut buffer);
        let y_shift = <dyn Read as ReadBytes>::read::<f32>(&mut buffer);
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
        let cache = HashMap::<u32, Vec<String>>::with_capacity(numentries as usize);
        let geo_data = GeoData {
            width,
            height,
            x_scale,
            y_scale,
            x_shift,
            y_shift,
            numentries,
        };

        Ok(Self {
            cache,
            buffer,
            geo_data,
        })
    }
}

#[path = "../tests/cache/cache.rs"]
#[cfg(test)]
mod tests;
