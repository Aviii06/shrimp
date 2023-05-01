// Struct pixel
#[derive(Clone)]
struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

impl Pixel {
    // Create a new pixel
    fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel { red, green, blue }
    }

    fn draw(&self) {
        print!(
            "\x1b[48;2;{};{};{}m  \x1b[0m",
            self.red, self.green, self.blue
        );
    }
}

// Struct Buffer
struct Buffer {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Buffer {
    // Create a new buffer
    fn new(width: usize, height: usize) -> Buffer {
        let pixels = vec![Pixel::new(0, 0, 0); width * height];
        Buffer {
            pixels,
            width,
            height,
        }
    }

    // Draw a pixel
    fn draw(&self) {
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

    fn set(&mut self, x: usize, y: usize, pixel: Pixel) {
        // Row-Major
        self.pixels[y * self.width + x] = pixel;
    }
}

fn main() {
    // Set background color to green using ansi for 10 characters
    let px = Pixel {
        red: 255,
        green: 0,
        blue: 0,
    };

    let mut buffer = Buffer::new(10, 10);
    for i in 0..10 {
        for j in 0..10 {
            buffer.set(i, j, px.clone());
        }
    }

    buffer.draw();
    // px.draw();
}
