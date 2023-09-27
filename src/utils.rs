use std::{error::Error, fs::File, io::Write};

use crate::structs::Imatge;

pub fn idx_to_coords(idx: usize, w: usize) -> (usize, usize) {
    (
        idx % w,
        idx / w,
    )
}

pub fn coords_to_idx(x: usize, y: usize, w: usize) -> usize {
    x + w*y
}

pub fn guardar_pixels(imatge: Imatge) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("sortida.ppm")?;
    let pixels = imatge.to_string();
    file.write_all(pixels.as_bytes())?;
    Ok(())
}
