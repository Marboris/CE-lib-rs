mod zstd {
    pub mod c;
}
mod lz4 {
    pub mod c;
}
mod xz2 {
    pub mod c;
}

mod compression;
mod decompression;
mod decryption;
mod encryption;

use decryption::decrypt_data;
use encryption::encrypt_data;

use compression::compress_and_save;
use decompression::decompress_from_file;

use chacha20poly1305::aead::{KeyInit, OsRng};
use chacha20poly1305::ChaCha20Poly1305;
use rand::Rng;
use std::io;

fn main() -> io::Result<()> {
    let input_data = "This is a test message to be encrypted and compressed.";
    println!("Original data: {}", input_data);

    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let nonce: [u8; 12] = rand::thread_rng().gen();
    println!("Encryption Key: {:?}", key);
    println!("Nonce: {:?}", nonce);

    let encrypted_data = encrypt_data(input_data.as_bytes(), &key, &nonce);
    println!("Encrypted data: {:?}", encrypted_data);

    let file_path = "compressed_data.mrs";

    let compressed_size = compress_and_save(&encrypted_data, file_path)?;
    println!("Data encrypted and compressed. Saved to {}", file_path);
    println!("Compressed size: {} bytes", compressed_size);
    println!("Original encrypted size: {} bytes", encrypted_data.len());

    let decompressed_data = decompress_from_file(file_path)?;
    println!("Data decompressed from {}", file_path);

    println!("Decryption Key: {:?}", key);
    println!("Nonce: {:?}", nonce);

    let decrypted_data = decrypt_data(&decompressed_data, &key, &nonce);
    println!(
        "Decrypted data: {}",
        String::from_utf8(decrypted_data).unwrap()
    );

    Ok(())
}
