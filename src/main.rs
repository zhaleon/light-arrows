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
use rendering::Camera;

fn main() {
    let fake_origin = DVec3{ x: 3.0, y: 6.0, z: 3.0 };
    let spheres = vec!(
        Sphere {
            center: fake_origin,
            radius: 0.33,
            material: Material { specular_color: DVec3::new(1.0, 1.0, 1.0) },
        },

        Sphere {
            center: fake_origin + DVec3::X,
            radius: 0.33,
            material: Material { specular_color: DVec3::new(1.0, 0.0, 0.0) },
        },
        Sphere {
            center: fake_origin + 2.0 * DVec3::X,
            radius: 0.33,
            material: Material { specular_color: DVec3::new(1.0, 0.0, 0.0) },
        },
        Sphere {
            center: fake_origin + 3.0 * DVec3::X,
            radius: 0.33,
            material: Material { specular_color: DVec3::new(1.0, 0.0, 0.0) },
        },

        Sphere {
            center: fake_origin + DVec3::Y,
            radius: 0.33,
            material: Material { specular_color: DVec3::new(0.0, 1.0, 0.0) },
        },
        Sphere {
            center: fake_origin + 2.0 * DVec3::Y,
            radius: 0.33,
            material: Material { specular_color: DVec3::new(0.0, 1.0, 0.0) },
        },
        Sphere {
            center: fake_origin + 3.0 * DVec3::Y,
            radius: 0.33,
            material: Material { specular_color: DVec3::new(0.0, 1.0, 0.0) },
        },

        Sphere {
            center: fake_origin + DVec3::Z,
            radius: 0.33,
            material: Material { specular_color: DVec3::new(0.0, 0.0, 1.0) },
        },
        Sphere {
            center: fake_origin + 2.0 * DVec3::Z,
            radius: 0.33,
            material: Material { specular_color: DVec3::new(0.0, 0.0, 1.0) },
        },
        Sphere {
            center: fake_origin + 3.0 * DVec3::Z,
            radius: 0.33,
            material: Material { specular_color: DVec3::new(0.0, 0.0, 1.0) },
        },
    );

    let image = rendering::render(
        Camera::default(),
        spheres,
    );
    
    fs::create_dir_all("out").unwrap();
    image.save("out/output.png").unwrap();
}
