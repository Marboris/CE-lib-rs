
mod encryption;
mod decryption;
mod decompression;
mod compression;

use encryption::encrypt_data;
use decryption::decrypt_data;

use decompression::decompress_from_file;
use compression::compress_and_save;

use chacha20poly1305::aead::{KeyInit, OsRng}; // برای تولید کلید و رمزنگاری
use chacha20poly1305::ChaCha20Poly1305; // ChaCha20Poly1305 استفاده از نوع 
use rand::Rng;
use std::io;


fn main() -> io::Result<()> {
    let input_data = "This is a test message to be encrypted and compressed.";
    println!("Original data: {}", input_data);

    // تولید کلید و nonce تصادفی
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let nonce: [u8; 12] = rand::thread_rng().gen();  // nonce با طول 12 بایت

    // چاپ کلید و nonce
    println!("Encryption Key: {:?}", key);
    println!("Nonce: {:?}", nonce);

    // رمزنگاری داده‌ها
    let encrypted_data = encrypt_data(input_data.as_bytes(), &key, &nonce);
    println!("Encrypted data: {:?}", encrypted_data);

    let file_path = "compressed_data.mrs";

    // فشرده‌سازی داده‌های رمزنگاری‌شده و ذخیره در فایل
    let compressed_size = compress_and_save(&encrypted_data, file_path)?;
    println!("Data encrypted and compressed. Saved to {}", file_path);
    println!("Compressed size: {} bytes", compressed_size);
    println!("Original encrypted size: {} bytes", encrypted_data.len());

    // رفع فشرده‌سازی و رمزگشایی داده‌ها
    let decompressed_data = decompress_from_file(file_path)?;
    println!("Data decompressed from {}", file_path);

    // چاپ کلید و nonce برای رمزگشایی
    println!("Decryption Key: {:?}", key);
    println!("Nonce: {:?}", nonce);

    let decrypted_data = decrypt_data(&decompressed_data, &key, &nonce);
    println!("Decrypted data: {}", String::from_utf8(decrypted_data).unwrap());

    Ok(())
}
