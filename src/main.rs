pub mod utils;
pub mod structs;

use std::{error::Error, sync::mpsc::{self, Receiver}, thread, collections::BinaryHeap, cmp::Reverse, io::BufWriter, fs::File};
use std::io::Write;

use rayon::prelude::*;
use structs::{Imatge, Color, Complex};

use indicatif::{ProgressBar, ProgressStyle};

use crate::{structs::Pixel, utils::guardar_pixels};

const IMATGE_WIDTH: usize = 10000;
const IMATGE_HEIGHT: usize = 8000;

const ITERACIONS_MAX: usize = 50;
const LENGTH_THRESHOLD: usize = 10000000;

const SCALING_FACTOR: f64 = (IMATGE_WIDTH / 2) as f64;

//const COLOR_DINS: Color = Color::new(255, 0, 0);
//const COLOR_FORA: Color = Color::new(0, 255, 0);
const COLOR_NO_CALCULAT: Color = Color::new(255, 255, 255);
const COLOR_DINS: Color = Color::new(184, 192, 224);
const COLOR_FORA: Color = Color::new(54, 58, 79);

fn main() -> Result<(), Box<dyn Error>> {
    let pixels = vec!(Pixel::default(); IMATGE_WIDTH * IMATGE_HEIGHT);
    let mut img = Imatge::new(IMATGE_WIDTH, IMATGE_HEIGHT, pixels);

    println!("Calculant...");

    let (sender, reciever) = mpsc::channel::<Pixel>();

    // Generar i enviar
    let t1 = thread::spawn(move || {
        img.pixels_mut().par_iter_mut().enumerate().for_each(|(i, p)| p.calcular(i, sender.clone()));
    });

    // Rebre i guardar
    let t2 = thread::spawn(|| {
        rebre_pixels_i_escriure_a_disk(reciever);
    });
    
    t1.join().unwrap();
    t2.join().unwrap();


    Ok(())
}

fn rebre_pixels_i_escriure_a_disk(receiver: Receiver<Pixel>) {
    let mut b_heap: BinaryHeap<Reverse<Pixel>> = BinaryHeap::new();
    let file = File::create("sortida.ppm").expect("No s'ha pogut obrir 'sortida.ppm'");
    let mut buffer = BufWriter::new(file);
    let mut next = 0;

    
    // ppm spec
    writeln!(buffer, "P3").unwrap();
    writeln!(buffer, "{} {}", IMATGE_WIDTH, IMATGE_HEIGHT).unwrap();
    writeln!(buffer, "255").unwrap();

    let progres = ProgressBar::with_style( // TODO: Make pretty
        ProgressBar::new((IMATGE_WIDTH * IMATGE_HEIGHT) as u64),
        ProgressStyle::default_bar()
    );

    while next < IMATGE_WIDTH * IMATGE_HEIGHT { 
        if let Ok(pixel_rebut) = receiver.recv() { // Err means sender hung up, m'és igual
            let index = pixel_rebut.index;
            if next != index {
                b_heap.push(Reverse(pixel_rebut));
                continue;
            }
            writeln!(buffer, "{}", pixel_rebut).unwrap();
            next += 1; progres.inc(1);
        }

        while let Some(Reverse(peek)) = b_heap.peek() {
            if peek.index != next { break; }

            writeln!(buffer, "{}", peek).unwrap();
            b_heap.pop();
            next += 1; progres.inc(1);
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
