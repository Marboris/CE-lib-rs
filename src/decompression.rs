
use std::fs::File;
use std::io::{self, Read};
use std::io::BufReader;

use crate::zstd::c::decompress_zstd;

// تابع برای خواندن داده‌ها از فایل
pub fn read_from_file(file_path: &str) -> io::Result<Vec<u8>> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut file_data = Vec::new();
    
    buf_reader.read_to_end(&mut file_data)?;  // خواندن همه داده‌ها از فایل
    Ok(file_data)
}






// تابع اصلی برای خواندن و رفع فشرده‌سازی داده‌ها از فایل
pub fn decompress_from_file(file_path: &str) -> io::Result<Vec<u8>> {
    // خواندن داده‌های فشرده از فایل
    let compressed_data = read_from_file(file_path)?;
    
    // رفع فشرده‌سازی داده‌ها
    Ok(decompress_zstd(&compressed_data))
}
