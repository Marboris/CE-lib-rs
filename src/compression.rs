use std::fs::File;
use std::io::{self, Write, Read};
use std::io::BufWriter;






// تابع برای ذخیره‌سازی داده‌ها در یک فایل
fn save_to_file(data: &[u8], file_path: &str) -> io::Result<u64> {
    let file = File::create(file_path)?;
    let mut buffered_writer = BufWriter::new(file);

    buffered_writer.write_all(data)?;  // نوشتن داده‌ها در فایل
    buffered_writer.flush()?;  // اطمینان از اینکه تمام داده‌ها نوشته شده‌اند

    // دریافت متادیتای فایل برای اندازه آن
    let file = buffered_writer.into_inner()?;
    let file_size = file.metadata()?.len();

    Ok(file_size)
}

// مثال استفاده از توابع فشرده‌سازی و ذخیره‌سازی
pub fn compress_and_save(input: &[u8], file_path: &str) -> io::Result<u64> {
    // فشرده‌سازی داده‌ها
    let compressed_data = compress_data(input)?;
    
    // ذخیره‌سازی داده‌های فشرده شده در فایل
    save_to_file(&compressed_data, file_path)
}
