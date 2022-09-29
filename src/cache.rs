use crate::io::io;
use crate::logger::LOGGER;
use crate::number::{ReadBytes, number};
use crate::types::Buffer;
use crate::types::Data;
use math::round;
use std::collections::HashMap;
use std::io::{Cursor, Read, Result, Seek, SeekFrom};
use std::io::{Error, ErrorKind};

#[derive(Debug, Default)]
pub struct GeoData {
    width: u32,
    height: u32,
    x_scale: f32,
    y_scale: f32,
    x_shift: f32,
    y_shift: f32,
    numentries: u32,
}

#[derive(Debug, Default)]
pub struct Cache {
    cache: Data,
    buffer: Buffer,
    geo_data: GeoData,
}

static mut HEADER_SIZE: u32 = 32;

#[allow(unused)]
impl Cache {
    pub fn lookup(&mut self, longitude: f32, latitude: f32) -> Vec<String> {
        debug!(LOGGER, "longitude: {}, latitude: {}", longitude, latitude);
        let index = self.lookup_uncached(longitude, latitude);
        return match self.lookup_entry(index) {
            Some(address) => address.to_vec(),
            None => {
                debug!(LOGGER, "Couldn't find any geospatial data for longitude: {} & latitude: {}", longitude, latitude);
                return vec![];
            }
        }
    }

    fn read_unsigned_i32(&mut self) -> Result<i32> {
        let mut value = 0 as i32;
        let mut bits = 0 as u32;
        loop {
            let position = self.buffer.position() as i64;
            let data = self.buffer.seek(SeekFrom::Current(position))
            .expect(format!("Can't find the position {} in the buffer", position).as_str()) as i32;
            value |= (data & 0x7F) << bits;
            if (data & 0x80) == 0 {
                return Ok(value);
            }
            bits += 7;
            if bits > 35 {
                Error::new(ErrorKind::InvalidData, "Variable length quantity is too long for expected integer");
            }
        }
        Error::new(ErrorKind::InvalidData, "An issue occurred while reading the integer");
    }

    fn lookup_uncached(&mut self, longitude: f32, latitude: f32) -> i32 {
        let x = round::floor(
            ((longitude + self.geo_data.x_shift) * self.geo_data.x_scale).into(),
            0,
        ) as i32;
        let y = round::floor(
            ((latitude + self.geo_data.y_shift) * self.geo_data.y_scale).into(),
            0,
        ) as i32;
        if x < 0 || number::parse_u32(x) >= self.geo_data.width || y < 0 || number::parse_u32(y) >= self.geo_data.height {
            return 0;
        }
        unsafe {
            let position = HEADER_SIZE + (number::parse_u32(y) << 2);
            let row_position = self
                .buffer
                .seek(SeekFrom::Current(position.into()))
                .expect(&format!("Couldn't find the position for {}", position))
                as i32;
            self.buffer.set_position(row_position as u64);
            let mut i = 0;
            while i < x {
                let count = self.read_unsigned_i32().unwrap();
                i += self.read_unsigned_i32().unwrap() + 1;
                if x < i {
                    return count;
                }
            }
            return 0;
        }
    }

    fn lookup_entry_uncached(&mut self, index: u32) -> Vec<String> {
        unsafe {
            let position = HEADER_SIZE + (self.geo_data.height << 2);
            let start = <dyn Read as ReadBytes>::read::<u32>(&mut self.buffer);
            let end = <dyn Read as ReadBytes>::read::<u32>(&mut self.buffer);
            if start == end {
                return vec![];
            }
            self.buffer.set_position(start as u64);
            debug!(LOGGER, "Here position {}, {}", start, end);
            // let decoded = str::from_utf8(self.buffer.seek(SeekFrom::Current(end)).unwrap());
            self.buffer.rewind();
            return vec![];
        }
    }

    fn lookup_entry(&mut self, key: i32) -> Option<&mut Vec<String>> {
        let index = number::parse_u32(key);
        if key < 0 || index >= self.geo_data.numentries {
            return None;
        }
        return match self.cache.contains_key(&index) {
            true => self.cache.get_mut(&index),
            false => {
                let cached_entry = self.lookup_entry_uncached(index);
                self
                .cache
                .insert(index, cached_entry).as_mut();
                // I can't return the result of insert, I'm getting
                // "cannot return reference to temporary value" error
                return self.cache.get_mut(&index);
            },
        };
    }

    pub unsafe fn parse_buffer(file_name: &String) -> Result<Self> {
        let mut buffer = Cursor::new(io::get_file_as_byte_vec(&file_name).unwrap());
        let magic_number = <dyn Read as ReadBytes>::read::<u32>(&mut buffer);
        if magic_number != 1845944321 {
            panic!("Index file does not have the correct type or version.")
        }
        let width = <dyn Read as ReadBytes>::read::<u32>(&mut buffer);
        let height = <dyn Read as ReadBytes>::read::<u32>(&mut buffer);
        let x_scale = <dyn Read as ReadBytes>::read::<f32>(&mut buffer);
        let y_scale = <dyn Read as ReadBytes>::read::<f32>(&mut buffer);
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
        debug!(LOGGER, "Cache size: {}", cache.capacity());
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
