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

    let data = [0u8; (4 * WIDTH * HEIGHT) as usize];
    writer.write_image_data(&data).unwrap();
}
