extern crate image;

use image::{ImageBuffer, Rgb};

const WIDTH: u32 = 10;
const HEIGHT: u32 = 10;

fn main() {
    // a default (black) image containing Rgb values
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(WIDTH, HEIGHT);

    // set a central pixel to white
    // image.get_pixel_mut(5, 5).data = [255, 255, 255];
    image.get_pixel_mut(5, 5).0 = [255, 255, 255];

    fs::create_dir_all("out").unwrap();
    image.save("out/output.png").unwrap();
}
