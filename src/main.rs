pub mod utils;
pub mod structs;

use std::{error::Error, sync::mpsc::{self, Receiver}, thread, collections::{BinaryHeap}, cmp::Reverse, io::{BufWriter, Write}, fs::File};
use rayon::prelude::*;

use structs::{Imatge, Color, Complex};
//use utils::guardar_pixels;

use crate::structs::Pixel;

const IMATGE_WIDTH: usize = 100000;
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

    println!("Calculant...");
    

    let (sender, reciever) = mpsc::channel::<Pixel>();

    // Generar i enviar
    thread::spawn(move || {
        img.pixels_mut().par_iter_mut().enumerate().for_each(|(i, p)| p.calcular(i, sender.clone()));
    });

    // Rebre i guardar
    thread::spawn(|| {
        rebre_pixels_i_escriure_a_disk(reciever);
    });
    

    println!("Guardant a disc...");
    //guardar_pixels(img)?;

    println!("Acabat!");
    Ok(())
}

fn rebre_pixels_i_escriure_a_disk(receiver: Receiver<Pixel>) {
    let mut next = 0;
    let mut b_heap: BinaryHeap<Reverse<Pixel>> = BinaryHeap::new();
    let file = File::create("sortida.ppm").expect("No s'ha pogut obrir 'sortida.ppm'");
    let mut buffer = BufWriter::new(file);
    
    // ppm spec
    writeln!(buffer, "P3").unwrap();
    writeln!(buffer, "{} {}", IMATGE_WIDTH, IMATGE_HEIGHT).unwrap();
    writeln!(buffer, "255").unwrap();

    while let Ok(pixel_rebut) = receiver.recv() {
        let index = pixel_rebut.index;
        if next != index {
            b_heap.push(Reverse(pixel_rebut));
            continue;
        }

        // El pixel que tenim és el que toca escriure
        writeln!(buffer, "{}", pixel_rebut).unwrap();

        next += 1;
        
        while let Some(Reverse(peek)) = b_heap.peek() {
            if peek.index != next {
                break;
            }

            // El pixel que hem trobat és el que toca escriure
            writeln!(buffer, "{}", peek).unwrap();
            next += 1;
        }
    }
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
