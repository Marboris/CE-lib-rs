use std::fs::File;
use std::io::BufReader;
use std::io::{self, Read};

use crate::zstd::c::decompress_zstd;

pub fn read_from_file(file_path: &str) -> io::Result<Vec<u8>> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut file_data = Vec::new();

    buf_reader.read_to_end(&mut file_data)?;
    Ok(file_data)
}

pub fn decompress_from_file(file_path: &str) -> io::Result<Vec<u8>> {
    let compressed_data = read_from_file(file_path)?;

    Ok(decompress_zstd(&compressed_data))
}
