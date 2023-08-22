extern crate image;

mod rendering;
mod surface;
mod space3d;

use std::fs;
use surface::VOID_SURFACE;

fn main() {

    let image = rendering::render(Default::default(), VOID_SURFACE);
    
    fs::create_dir_all("out").unwrap();
    image.save("out/output.png").unwrap();
}
