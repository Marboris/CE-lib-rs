use std::io::{self, Read, Write};
use xz2::write::XzEncoder;
use xz2::read::XzDecoder;

// تابع برای فشرده‌سازی داده‌ها با xz2
pub fn compress_data_xz2(input: &[u8], compression_level: Option<u32>) -> Vec<u8> {
    // تعیین سطح فشرده‌سازی
    let level = match compression_level {
        Some(level) if (0..=9).contains(&level) => level, // بررسی سطح فشرده‌سازی بین 0 تا 9
        _ => 6, // استفاده از سطح فشرده‌سازی پیش‌فرض در صورت نامعتبر بودن ورودی
    };

    let mut compressed_data = Vec::new();

    // تلاش برای فشرده‌سازی داده‌ها
    let result = (|| {
        let mut encoder = XzEncoder::new(Vec::new(), level);
        encoder.write_all(input)?;  // نوشتن داده‌ها برای فشرده‌سازی
        encoder.flush()?;  // مطمئن شوید که داده‌ها به طور کامل نوشته شده‌اند
        compressed_data = encoder.finish()?;  // پایان عملیات و گرفتن داده‌های فشرده‌شده
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

// تابع برای رفع فشرده‌سازی داده‌ها با xz2
pub fn decompress_data_xz2(input: &[u8]) -> Vec<u8> {
    let mut decompressed_data = Vec::new();

    // تلاش برای رفع فشرده‌سازی داده‌ها
    let result = (|| {
        let mut decoder = XzDecoder::new(input);
        decoder.read_to_end(&mut decompressed_data)?;  // خواندن تمام داده‌های فشرده‌شده
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
