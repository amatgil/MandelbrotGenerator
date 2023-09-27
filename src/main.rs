pub mod utils;
pub mod structs;

use std::error::Error;
use rayon::prelude::*;

use structs::{Imatge, Color, Complex};
use utils::guardar_pixels;

use crate::structs::Pixel;

const IMATGE_WIDTH: usize = 10000;
const IMATGE_HEIGHT: usize = IMATGE_WIDTH;

const ITERACIONS_MAX: usize = 50;
const LENGTH_THRESHOLD: usize = 10000000;

const SCALING_FACTOR: f64 = (IMATGE_WIDTH / 2) as f64;

const COLOR_DINS: Color = Color::new(255, 0, 0);
const COLOR_FORA: Color = Color::new(0, 255, 0);
const COLOR_NO_CALCULAT: Color = Color::new(255, 255, 255);

fn main() -> Result<(), Box<dyn Error>> {
    let pixels = vec!(Pixel::default(); IMATGE_WIDTH * IMATGE_HEIGHT);
    let mut img = Imatge::new(IMATGE_WIDTH, IMATGE_HEIGHT, pixels);

    let pixels = img.pixels_mut();

    println!("Calculant...");
    pixels.par_iter_mut().enumerate().for_each(|(i, p)| p.calcular(i));

    println!("Guardant a disc...");
    guardar_pixels(img)?;

    println!("Acabat!");
    Ok(())
}


fn mandel_equation(x: usize, y: usize) -> bool {
    let x = (x as f64 - (IMATGE_WIDTH / 2) as f64) / SCALING_FACTOR ;
    let y = (y as f64 - (IMATGE_HEIGHT / 2) as f64) / SCALING_FACTOR ;

    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex { re: x, im: y };

    let eqn = |z| z*z + c;

    for _ in 0..ITERACIONS_MAX {
        z = eqn(z);
    }

    z.length() < LENGTH_THRESHOLD as f64
}
