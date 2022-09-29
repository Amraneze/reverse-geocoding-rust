pub mod io {
    use std::fs::read;
    use std::io::Result;

    pub fn get_file_as_byte_vec(filename: &String) -> Result<Vec<u8>> {
        match read(filename) {
            Ok(bytes) => Ok(bytes),
            Err(e) => panic!(
                "An error occurred while reading file {} with exception {}",
                filename, e
            ),
        }
    }
}
