use std::collections::HashMap;
use std::io::Cursor;

pub type Data = HashMap<u32, Vec<String>>;
pub type Buffer = Cursor<Vec<u8>>;
