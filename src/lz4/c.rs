use std::io::{self, Read, Write};

use lz4::{EncoderBuilder, Decoder};

// lz4
// سطح فشرده‌سازی (در lz4 چندان سطحی مشابه zstd وجود ندارد اما می‌توان EncoderBuilder را پیکربندی کرد)

// تابع برای فشرده‌سازی داده‌ها با lz4
pub fn compress_lz4(input: &[u8]) -> Vec<u8> {
    let mut compressed_data = Vec::new();

    // تلاش برای فشرده‌سازی داده‌ها
    let result = (|| {
        let mut encoder = EncoderBuilder::new().build(&mut compressed_data)?;
        encoder.write_all(input)?;  // نوشتن داده‌ها برای فشرده‌سازی
        let (_output, result) = encoder.finish();  // پایان فرآیند فشرده‌سازی
        result?;  // بررسی و برگشت نتیجه عملیات
        Ok::<(), io::Error>(())  // در صورت موفقیت، مقدار Ok برگردانده می‌شود
    })();

    // اگر هر گونه خطایی رخ داد، داده اصلی را بازگردانید
    match result {
        Ok(_) => compressed_data,  // اگر فشرده‌سازی موفق بود، داده فشرده‌شده را بازگردانید
        Err(_) => {
            println!("Compression failed. Returning original input data.");
            input.to_vec()  // اگر خطا رخ داد، داده اصلی را بازگردانید
        }
    }
}

// تابع برای رفع فشرده‌سازی داده‌ها با lz4
pub fn decompress_lz4(input: &[u8]) -> Vec<u8> {
    let mut decompressed_data = Vec::new();

    // تلاش برای رفع فشرده‌سازی داده‌ها
    let result = (|| {
        let mut decoder = Decoder::new(input)?;
        decoder.read_to_end(&mut decompressed_data)?;  // خواندن داده‌ها و رفع فشرده‌سازی
        Ok::<(), io::Error>(())  // در صورت موفقیت، مقدار Ok برگردانده می‌شود
    })();

    // اگر هر گونه خطایی رخ داد، داده اصلی (فشرده) را بازگردانید
    match result {
        Ok(_) => decompressed_data,  // اگر رفع فشرده‌سازی موفق بود، داده رفع فشرده‌شده را بازگردانید
        Err(_) => {
            println!("Decompression failed. Returning original input data.");
            input.to_vec()  // اگر خطا رخ داد، داده اصلی (فشرده) را بازگردانید
        }
    }
}