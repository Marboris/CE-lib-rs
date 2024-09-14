use std::io::{self, Read, Write};

use zstd::stream::{Decoder, Encoder};

pub fn compress_zstd(input: &[u8], compression_level: Option<i32>) -> Vec<u8> {
    let level = match compression_level {
        Some(level) if (0..=22).contains(&level) => level,
        _ => 0,
    };

    let mut compressed_data = Vec::new();
    let result = (|| {
        let mut encoder = Encoder::new(&mut compressed_data, level)?;
        encoder.write_all(input)?;
        encoder.finish()?;
        Ok::<(), io::Error>(())
    })();

    match result {
        Ok(_) => compressed_data,
        Err(_) => {
            println!("[zstd] Compression failed. Returning original input data.");
            input.to_vec()
        }
    }
}

pub fn decompress_zstd(input: &[u8]) -> Vec<u8> {
    let mut decompressed_data = Vec::new();

    let result = (|| {
        let mut decoder = Decoder::new(input)?;
        decoder.read_to_end(&mut decompressed_data)?;
        Ok::<(), io::Error>(())
    })();

    match result {
        Ok(_) => decompressed_data,
        Err(_) => {
            println!("[zstd] Decompression failed. Returning original input data.");
            input.to_vec()
        }
    }
}
