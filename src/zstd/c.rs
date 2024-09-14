use std::io::{self, Read, Write};

use zstd::stream::{Encoder, Decoder};

// zstd
pub fn compress_zstd(input: &[u8], compression_level: Option<i32>) -> Vec<u8> {
    // تعیین سطح فشرده‌سازی
    let level = match compression_level {
        Some(level) if (0..=22).contains(&level) => level, // بررسی سطح فشرده‌سازی بین 0 تا 22
        _ => 0, // استفاده از سطح فشرده‌سازی پیش‌فرض در صورت نامعتبر بودن ورودی
    };

    // تلاش برای فشرده‌سازی داده‌ها
    let mut compressed_data = Vec::new();
    let result = (|| {
        let mut encoder = Encoder::new(&mut compressed_data, level)?;  // ایجاد encoder
        encoder.write_all(input)?;  // نوشتن داده‌ها برای فشرده‌سازی
        encoder.finish()?;  // پایان فرآیند فشرده‌سازی
        Ok::<(), io::Error>(())  // در صورت موفقیت، مقدار Ok برگردانده می‌شود
    })();

    // اگر هر گونه خطایی رخ داد، داده اصلی برگردانده می‌شود
    match result {
        Ok(_) => compressed_data,  // اگر فشرده‌سازی موفق بود، داده فشرده‌شده را بازگردانید
        Err(_) => {
            println!("[zstd] Compression failed. Returning original input data.");
            input.to_vec()  // اگر خطا رخ داد، داده اصلی را بازگردانید
        }
    }
}

pub fn decompress_zstd(input: &[u8]) -> Vec<u8> {
    let mut decompressed_data = Vec::new();

    // تلاش برای رفع فشرده‌سازی داده‌ها
    let result = (|| {
        let mut decoder = Decoder::new(input)?;  // ایجاد decoder
        decoder.read_to_end(&mut decompressed_data)?;  // خواندن داده‌ها و رفع فشرده‌سازی
        Ok::<(), io::Error>(())  // در صورت موفقیت، مقدار Ok برگردانده می‌شود
    })();

    // اگر هر گونه خطایی رخ داد، داده اصلی (فشرده) برگردانده می‌شود
    match result {
        Ok(_) => decompressed_data,  // اگر رفع فشرده‌سازی موفق بود، داده رفع فشرده‌شده را بازگردانید
        Err(_) => {
            println!("[zstd] Decompression failed. Returning original input data.");
            input.to_vec()  // اگر خطا رخ داد، داده اصلی (فشرده) را بازگردانید
        }
    }
}