use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

const OUTPUT_FILENAME: &str = "Disk.img";
const BUFFER_SIZE: usize = 8192;
const SECTOR_SIZE: usize = 512;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut output_file = File::create(OUTPUT_FILENAME)?;
    let mut number_of_sector = vec![];

    for filename in &args[1..] {
        println!("Concatenate {filename} to {OUTPUT_FILENAME}");

        let bytes_read = concatenate_file(&mut output_file, filename).unwrap();
        let bytes_pad = pad_with_zero(&mut output_file, bytes_read).unwrap();
        number_of_sector.push(((bytes_read + bytes_pad) / SECTOR_SIZE) as u16);
    }

    let number_of_copied_sector: u16 = number_of_sector.iter().sum();
    println!("Number of copied sector: {}", number_of_copied_sector - 1);
    set_number_of_sector(
        &mut output_file,
        number_of_copied_sector - 1,
        number_of_sector[1],
    )?;

    Ok(())
}

fn concatenate_file(to: &mut File, from: &str) -> std::io::Result<usize> {
    let mut from = File::open(from)?;
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut total_bytes_read = 0;

    loop {
        let bytes_read = from.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        to.write_all(&buffer[..bytes_read])?;
        total_bytes_read += bytes_read;
    }

    Ok(total_bytes_read)
}

fn pad_with_zero(file: &mut File, bytes_read: usize) -> std::io::Result<usize> {
    let bytes_read = bytes_read % SECTOR_SIZE;
    if bytes_read == 0 {
        return Ok(0);
    }

    let zeros = [0u8; SECTOR_SIZE];
    let bytes_pad = SECTOR_SIZE - bytes_read;
    file.write_all(&zeros[..bytes_pad])?;

    Ok(bytes_pad)
}

fn set_number_of_sector(
    file: &mut File,
    number_of_total_sector: u16,
    number_of_kernel32_sector: u16,
) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(5))?;
    file.write_all(&number_of_total_sector.to_le_bytes())?;
    file.write_all(&number_of_kernel32_sector.to_le_bytes())?;

    Ok(())
}
