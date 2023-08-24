extern crate image;

mod rendering;
mod surface;
mod ray;

use std::fs;
use glam::DVec3;
use surface::Sphere;

fn main() {
    let sphere = Sphere {
        center: DVec3{ x: 2.0, y: 5.0, z: 3.0 },
        radius: 1.0
    };
    let image = rendering::render(Default::default(), sphere);
    
    fs::create_dir_all("out").unwrap();
    image.save("out/output.png").unwrap();
}
