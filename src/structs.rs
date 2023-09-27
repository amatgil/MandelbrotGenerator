use std::{fmt::Display, ops::{Add, Mul}, sync::mpsc::Sender, cmp::Ordering};

use crate::{utils::{coords_to_idx, idx_to_coords}, IMATGE_WIDTH, mandel_equation, COLOR_DINS, COLOR_FORA, COLOR_NO_CALCULAT};

#[derive(Clone)]
pub struct Imatge {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>
}

impl Imatge {
    pub fn new(width: usize, height: usize, pixels: Vec<Pixel>) -> Self {
        Self { width, height, pixels }
    }
    pub fn pixels_mut(&mut self) -> &mut Vec<Pixel> {
        &mut self.pixels
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Estat {
    Dins,
    Fora,
    NoCalculat
}

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    estat: Estat,
    pub index: usize,
}

impl Default for Pixel {
    fn default() -> Self {
        Self { estat: Estat::NoCalculat, index: 10000000 }
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

}
impl Default for Color {
    fn default() -> Self {
        Self::new(0, 0, 0) // Negre
    }
}

impl Pixel {
    pub fn calcular(&mut self, idx: usize, sender: Sender<Pixel>) {
        self.index = idx;

        let (x, y) = idx_to_coords(idx, IMATGE_WIDTH);
        if mandel_equation(x, y) { self.estat = Estat::Dins; }
        else { self.estat = Estat::Fora;}

        sender.send(*self).expect("No s'ha pogut enviar el pixel al buffer");
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
impl PartialOrd for Pixel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl Eq for Pixel {}

impl Ord for Pixel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}


/// Aquí és on s'especifica com es generarà el ppm a partir d'un `Pixels` si se fa de cop
impl Display for Imatge { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        // Especificacions
        out.push_str("P3\n");                                       // Imatge amb colors
        out.push_str(&format!("{} {}\n", self.width, self.height)); // Mida
        out.push_str("255\n");                                        // Valor màx per pixel

        // Els pixels en si
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = coords_to_idx(x, y, self.width);
                out.push_str(&self.pixels[idx].to_string());
                out.push('\n');
            }
        }


        write!(f, "{}", out)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>3} {:>3} {:>3}", self.r, self.g, self.b)
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = match self.estat {
            Estat::Dins => COLOR_DINS,
            Estat::Fora => COLOR_FORA,
            Estat::NoCalculat => COLOR_NO_CALCULAT,
        };

        write!(f, "{}", color)
    }
}

#[derive(Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn length(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - (self.im * rhs.im),
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}




