use glam::DVec3;
use image::{RgbImage, Rgb};
use crate::{surface::{Sphere, Surface}, ray::{Ray, HitInfo}, sky::sky_light_in_direction};

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
            // resolution_v: 360,
            // resolution_h: 480,
            resolution_v: 720,
            resolution_h: 960,
        }
    }
}

pub fn rays_of(camera: Camera) -> impl Iterator<Item=(u32, u32, Ray)> {
    let height = camera.resolution_h as f64;
    let width  = camera.resolution_v as f64;
    let tan_theta = (camera.fov_h / 2.0).tan();

    (0..camera.resolution_v).flat_map( move |row| {
        (0..camera.resolution_h).map( move |col| {
            (
                col,
                row,
                Ray {
                    origin: camera.center,
                    direction: DVec3 {
                        x: - tan_theta
                            + (2 * col + 1) as f64 * tan_theta / width,
                        y: 1.0,
                        z: height / width * tan_theta
                            - (2 * row + 1) as f64 * tan_theta / width,
                    }.normalize_or_zero(),
                }
            )
        })
    })
}

// returns the first sphere hit
fn hit(ray: Ray, spheres: &Vec<Sphere>) -> Option<HitInfo> {
    // (contact point, contact normal)
    spheres
        .iter()
        .filter_map(|sphere| sphere.intersect(ray))
        .min_by(|hit1, hit2| {
            (&hit1.contact_time)
                .partial_cmp(&hit2.contact_time)
                .unwrap() // TODO: skull
        })
}

// fn hit_sequence(ray: Ray

// fn shoot_ray(ray: Ray, spheres: &Vec<Sphere>) -> Option<HitInfo> {
//     for bounce in 0..BOUNCE_LIMIT {

//     }
//     hit(ray, &spheres)
// }

fn light_to_color(light: DVec3) -> Rgb<u8> {
    let r = (255.0 * light.x) as u8;
    let g = (255.0 * light.y) as u8;
    let b = (255.0 * light.z) as u8;
    let color = Rgb([r, g, b]);
    return color;
}

pub fn render(camera: Camera, objects: Vec<Sphere>) -> RgbImage {
    let mut image = RgbImage::new(camera.resolution_h, camera.resolution_v);
    for (row, col, ray) in rays_of(camera) {
        let intersect_info = hit(ray, &objects);

        let pixel = match intersect_info {
            Some(intersect_info) => {
                // let specular_direction = ray.direction.project_onto(intersect_info.contact_normal) 
                    // - ray.direction.reject_from(intersect_info.contact_normal);

                let specular_direction = ray.direction - 2.0 * (ray.direction.dot(intersect_info.contact_normal)) * intersect_info.contact_normal;

                intersect_info.material * sky_light_in_direction(specular_direction)
            },

            None => sky_light_in_direction(ray.direction),
        };

        image.put_pixel(row, col, light_to_color(pixel));
    }
    image
}
