
use chacha20poly1305::aead::{Aead, KeyInit}; // برای تولید کلید و رمزنگاری
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce}; // ChaCha20Poly1305 استفاده از نوع 

// تابع برای رمزگشایی داده‌ها با ChaCha20Poly1305
pub fn decrypt_data(encrypted_data: &[u8], key: &Key, nonce: &[u8; 12]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new(key);
    cipher.decrypt(Nonce::from_slice(nonce), encrypted_data).expect("decryption failure!")  // رمزگشایی داده‌ها
}