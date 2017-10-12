extern crate png;
extern crate num;

use std::f32;
use num::complex::*;
use std::io::*;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png::HasParameters;

const MAX_ITERS: u32 = 350;

const WIDTH: u32 = 8192;
const HEIGHT: u32 = 8192;

const CENTER_X: f32 = -0.74529;
const CENTER_Y: f32 = 0.113075;
const R: f32 = 1.5E-4;

fn main() {
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    const ARRAY_LEN: usize = (3 * WIDTH * HEIGHT) as usize;
    let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut data = vec![0u8; ARRAY_LEN];

    let LEFT_TOP_X: f32 = CENTER_X - R;
    let LEFT_TOP_Y: f32 = CENTER_Y + R;
    let RIGHT_BOTTOM_X: f32 = CENTER_X + R;
    let RIGHT_BOTTOM_Y: f32 = CENTER_Y - R;

    let math_width: f32 = (LEFT_TOP_X - RIGHT_BOTTOM_X).abs();
    let math_height: f32 = (LEFT_TOP_Y - RIGHT_BOTTOM_Y).abs();

    let mut percent_done = 0;
    for w in 0..WIDTH {
        for h in 0..HEIGHT {
            let pointer = ((w + WIDTH * h) * 3) as usize;
            let x = LEFT_TOP_X + ((w as f32) / (WIDTH as f32)) * math_width;
            let y = LEFT_TOP_Y - ((HEIGHT - h) as f32) / (HEIGHT as f32) * math_height;
            let val = val(x, y);
            assert!(val >= 0f32);
            assert!(val <= 1f32);
            let r = (255f32 * val) as u8;
            let g = (255f32 * val) as u8;
            let b = (255f32 * val) as u8;
            data[pointer] = r;
            data[pointer + 1] = g;
            data[pointer + 2] = b;
        }

        let pd = w * 1000 / WIDTH;
        if pd > percent_done {
            percent_done = pd;
            print!(".");
            std::io::stdout().flush().unwrap();
            if pd % 100 == 0 {
                print!(" {}%\n", pd / 10);
            }
        }
    }
    print!(" {}%\n", 100);
    writer.write_image_data(&data).unwrap();
}

fn val(x: f32, y: f32) -> f32 {
    let iters = get_iterations(x, y);
    if let Some(its) = iters {
        (its as f32 / MAX_ITERS as f32).powf(0.93)
    } else {
        1.0
    }
}

fn get_iterations(x: f32, y: f32) -> Option<u32> {
    let z0 = Complex32::new(0.0, 0.0);
    let c = Complex32::new(x, y);
    let mut z = z0;
    for i in 1..MAX_ITERS {
        z = z.powf(2.0) + c;
        if z.norm() > 2.0 {
            return Some(i);
        }
    }
    None
}