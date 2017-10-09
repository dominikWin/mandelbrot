extern crate png;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png::HasParameters;

fn main() {
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    const WIDTH: u32 = 64;
    const HEIGHT: u32 = 64;

    let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut data = [0u8; (4 * WIDTH * HEIGHT) as usize];
    for w in 0..WIDTH {
        for h in 0..HEIGHT {
            let pointer = ((w + WIDTH * h) * 4) as usize;
            let x = ((w as f32) / (WIDTH as f32)) * 2f32 - 1f32;
            let y = (((HEIGHT - h) as f32) / (HEIGHT as f32)) * 2f32 - 1f32;
            assert!(x >= -1f32);
            assert!(y >= -1f32);
            assert!(x <= 1f32);
            assert!(y <= 1f32);
            let val = val(x, y);
            assert!(val >= 0f32);
            assert!(val <= 1f32);
            let r = (255f32 * val) as u8;
            let g = (255f32 * val) as u8;
            let b = (255f32 * val) as u8;
            data[pointer] = r;
            data[pointer + 1] = g;
            data[pointer + 2] = b;
            data[pointer + 3] = 255;
        }
    }
    writer.write_image_data(&data).unwrap();
}

fn val(x: f32, y: f32) -> f32 {
    if x < 0.0 {0.0} else {if y < 0.0 {0.25} else {0.75}}
}
