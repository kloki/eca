use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use image::{ImageBuffer, ImageError, Rgb};
const BLACK: image::Rgb<u8> = Rgb([0u8; 3]);
const WHITE: image::Rgb<u8> = Rgb([255u8; 3]);

const SYMBOL_A: &[u8] = "█".as_bytes();
const SYMBOL_B: &[u8] = " ".as_bytes();
const SYMBOL_NEW_LINE: &[u8] = "\n".as_bytes();

pub fn to_png(filename: &str, iterations: Vec<Vec<bool>>) -> Result<(), ImageError> {
    let mut imgbuf = ImageBuffer::new(iterations[0].len() as u32, iterations.len() as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if iterations[y as usize][x as usize] {
            *pixel = BLACK
        } else {
            *pixel = WHITE
        }
    }

    imgbuf.save(filename)?;
    Ok(())
}

pub fn to_txt(filename: &str, iterations: Vec<Vec<bool>>) -> io::Result<()> {
    let file = File::create(filename)?;

    let mut writer = BufWriter::new(file);
    for i in iterations {
        for p in i {
            if p {
                writer.write_all(SYMBOL_A)?
            } else {
                writer.write_all(SYMBOL_B)?
            }
        }
        writer.write_all(SYMBOL_NEW_LINE)?
    }
    writer.flush()?;
    Ok(())
}
