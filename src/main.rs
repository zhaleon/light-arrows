extern crate image;

use std::fs;
use image::{Rgb, RgbImage};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

fn main() {
    let mut image = RgbImage::new(WIDTH, HEIGHT);

    for x in 0u8..=255 {
        for y in 0u8..=255 {
            image.put_pixel(x.into(), y.into(), Rgb([0, x, y]));
        }
    }

    fs::create_dir_all("out").unwrap();
    image.save("out/output.png").unwrap();
}
