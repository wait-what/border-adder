use image::Rgba;
use std::env;

fn main() {
    let path = env::args().nth(1).expect("Please provide a path");
    let mut image = image::open(&path).expect("Failed to open image").to_rgba8();

    for x in 0..image.width() {
        for y in 0..(image.height() / 30) {
            image.put_pixel(x, y, Rgba([0x20, 0x20, 0x20, 0xff]));
        }
    }

    image.save(path).expect("Failed to save image");
}
