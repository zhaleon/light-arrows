extern crate image;

mod rendering;
mod surface;
mod ray;
mod sky;
mod material;

use std::fs;
use glam::DVec3;
use image::Rgb;
use material::Material;
use surface::Sphere;

fn main() {
    let fake_origin = DVec3{ x: 3.0, y: 6.0, z: 3.0 };
    let spheres = vec!(
        Sphere {
            center: fake_origin,
            radius: 0.33,
            material: Material { color: Rgb([255, 255, 255]) },
        },
        Sphere {
            center: fake_origin + DVec3::X,
            radius: 0.33,
            material: Material { color: Rgb([255, 0, 0]) },
        },
        Sphere {
            center: fake_origin + DVec3::Y,
            radius: 0.33,
            material: Material { color: Rgb([0, 255, 0]) },
        },
        Sphere {
            center: fake_origin + DVec3::Z,
            radius: 0.33,
            material: Material { color: Rgb([0, 0, 255]) },
        },
    );
    let image = rendering::render(
        Default::default(),
        spheres,
    );
    
    fs::create_dir_all("out").unwrap();
    image.save("out/output.png").unwrap();
}
