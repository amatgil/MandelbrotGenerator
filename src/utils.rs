
pub fn idx_to_coords(idx: usize, w: usize) -> (usize, usize) {
    (
        idx % w,
        idx / w,
    )
}

pub fn coords_to_idx(x: usize, y: usize, w: usize) -> usize {
    x + w*y
}

use std::{error::Error, fs::File, io::Write};

use crate::structs::Imatge;
pub fn guardar_pixels(imatge: Imatge) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("sortida2.ppm")?;
    let pixels = imatge.to_string();
    file.write_all(pixels.as_bytes())?;
    Ok(())
}
