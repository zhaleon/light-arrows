extern crate image;

mod rendering;
mod surface;
mod space3d;

use std::fs;
use space3d::Point3;
use surface::Sphere;

fn main() {
    let sphere = Sphere {
        center: Point3{ x: 0.3, y: 2.0, z: 0.8 },
        radius: 1.0
    };
    let image = rendering::render(Default::default(), sphere);
    
    fs::create_dir_all("out").unwrap();
    image.save("out/output.png").unwrap();
}
