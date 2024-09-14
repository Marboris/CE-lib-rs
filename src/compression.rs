use std::fs::File;
use std::io::BufWriter;
use std::io::{self, Read, Write};

fn save_to_file(data: &[u8], file_path: &str) -> io::Result<u64> {
    let file = File::create(file_path)?;
    let mut buffered_writer = BufWriter::new(file);

    buffered_writer.write_all(data)?;
    buffered_writer.flush()?;
    let file = buffered_writer.into_inner()?;
    let file_size = file.metadata()?.len();

    Ok(file_size)
}

pub fn compress_and_save(input: &[u8], file_path: &str) -> io::Result<u64> {
    let compressed_data = compress_data(input)?;

    save_to_file(&compressed_data, file_path)
}
