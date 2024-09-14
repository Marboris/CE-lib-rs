use std::io::{self, Read, Write};

use lz4::{Decoder, EncoderBuilder};

pub fn compress_lz4(input: &[u8]) -> Vec<u8> {
    let mut compressed_data = Vec::new();

    let result = (|| {
        let mut encoder = EncoderBuilder::new().build(&mut compressed_data)?;
        encoder.write_all(input)?;
        let (_output, result) = encoder.finish();
        result?;
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

pub fn decompress_lz4(input: &[u8]) -> Vec<u8> {
    let mut decompressed_data = Vec::new();

    let result = (|| {
        let mut decoder = Decoder::new(input)?;
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
