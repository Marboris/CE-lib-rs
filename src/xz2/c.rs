use std::io::{self, Read, Write};
use xz2::read::XzDecoder;
use xz2::write::XzEncoder;

pub fn compress_data_xz2(input: &[u8], compression_level: Option<u32>) -> Vec<u8> {
    let level = match compression_level {
        Some(level) if (0..=9).contains(&level) => level,
        _ => 6,
    };

    let mut compressed_data = Vec::new();

    let result = (|| {
        let mut encoder = XzEncoder::new(Vec::new(), level);
        encoder.write_all(input)?;
        encoder.flush()?;
        compressed_data = encoder.finish()?;
        Ok::<(), io::Error>(())
    })();

    match result {
        Ok(_) => compressed_data,
        Err(_) => {
            println!("Compression failed. Returning original input data.");
            input.to_vec()
        }
    }
}

pub fn decompress_data_xz2(input: &[u8]) -> Vec<u8> {
    let mut decompressed_data = Vec::new();

    let result = (|| {
        let mut decoder = XzDecoder::new(input);
        decoder.read_to_end(&mut decompressed_data)?;
        Ok::<(), io::Error>(())
    })();

    match result {
        Ok(_) => decompressed_data,
        Err(_) => {
            println!("Decompression failed. Returning original input data.");
            input.to_vec()
        }
    }
}
