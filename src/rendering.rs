use glam::DVec3;
use image::{RgbImage, Rgb};
use crate::{surface::{Sphere, Surface}, ray::Ray};

pub struct Camera {
    center: DVec3,
    // assuming direction = <0, 1, 0>
    // TODO: parameterize yaw, pitch, roll
    fov_h: f64,
    viewport_distance: f64,
    resolution_v: u32,
    resolution_h: u32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            center: DVec3::ZERO,
            fov_h: 103_f64.to_radians(),
            viewport_distance: 1.0,
            resolution_v: 360,
            resolution_h: 480,
        }
    }
}

pub fn rays_of(camera: Camera) -> impl Iterator<Item=(u32, u32, Ray)> {
    let height = camera.resolution_h as f64;
    let width  = camera.resolution_v as f64;
    let d = camera.viewport_distance;
    let tan_theta = (camera.fov_h / 2.0).tan();

    (0..camera.resolution_h).flat_map( move |row| {
        (0..camera.resolution_v).map( move |col| {
            (
                row,
                col,
                Ray {
                    origin: camera.center,
                    direction: d * DVec3 {
                        x: - tan_theta
                            + (2 * col + 1) as f64 * tan_theta / width,
                        y: d,
                        z: d * height / width * tan_theta
                            - (2 * row + 1) as f64 * tan_theta / width,
                    }.normalize_or_zero(),
                }
            )
        })
    })
}

pub fn render(camera: Camera, sphere: Sphere) -> RgbImage {
    let mut image = RgbImage::new(camera.resolution_h, camera.resolution_v);
    for (row, col, ray) in rays_of(camera) {
        let pixel = Rgb(
            if sphere.intersect(ray).is_some() {
                [255, 255, 255]
            } else {
                [0, 0, 0]
            }
        );
        image.put_pixel(row, col, pixel);
    }
    image
}
