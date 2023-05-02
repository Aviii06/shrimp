use image::*;
use std::env;

// Struct pixel
#[derive(Clone)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Pixel {
    // Create a new pixel
    pub fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel { red, green, blue }
    }

    pub fn draw(&self) {
        print!(
            "\x1b[48;2;{};{};{}m  \x1b[0m",
            self.red, self.green, self.blue
        );
    }
}

// Struct Buffer
pub struct Buffer {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Buffer {
    // Create a new buffer
    pub fn new(width: usize, height: usize) -> Buffer {
        let pixels = vec![Pixel::new(0, 0, 0); width * height];
        Buffer {
            pixels,
            width,
            height,
        }
    }

    // Draw a pixel
    pub fn draw(&self) {
        // for pixel in &self.pixels {
        //     pixel.draw();
        // }
        for i in 0..self.height {
            for j in 0..self.width {
                let pixel = &self.pixels[i * self.width + j];
                pixel.draw();
            }
            println!();
        }
    }

    pub fn set(&mut self, x: usize, y: usize, pixel: Pixel) {
        // Row-Major
        self.pixels[y * self.width + x] = pixel;
    }
}

fn convert_png_to_buffer(string: &str) -> Buffer {
    let img = image::open(string).unwrap();
    let (width, height) = img.dimensions();
    let ratio = width as f32 / height as f32;
    let max_width = 70;
    let max_height = (max_width as f32 / ratio) as u32;

    let img = img.resize(max_width, max_height, image::imageops::FilterType::Nearest);

    let (width, height) = img.dimensions();
    let mut buffer = Buffer::new(width as usize, height as usize);
    // print!("{} {}", width, height);
    for (x, y, pixel) in img.pixels() {
        let px = Pixel::new(pixel[0], pixel[1], pixel[2]);
        buffer.set(x as usize, y as usize, px);
    }
    buffer
}

// take in command line argument in binary
fn main() {
    let args: Vec<String> = env::args().collect();
    let string = &args[1];
    let buffer = convert_png_to_buffer(string);

    buffer.draw();
}
