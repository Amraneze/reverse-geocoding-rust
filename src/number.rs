use std::io::Read;

pub trait FromBeBytes {
    fn from_be_bytes(bytes: [u8; 4]) -> Self;
}

impl FromBeBytes for u32 {
    fn from_be_bytes(bytes: [u8; 4]) -> Self {
        return u32::from_be_bytes(bytes);
    }
}

impl FromBeBytes for f32 {
    fn from_be_bytes(bytes: [u8; 4]) -> Self {
        return f32::from_be_bytes(bytes);
    }
}

pub trait ReadBytes {
    fn read<T: FromBeBytes>(buffer: &mut impl Read) -> T;
}

impl ReadBytes for dyn Read {
    fn read<T: FromBeBytes>(buffer: &mut impl Read) -> T {
        let mut number_buffer = [0; 4];
        buffer
            .read(&mut number_buffer)
            .expect("An error occurred while reading bytes from the buffer");
        return T::from_be_bytes(number_buffer);
    }
}
