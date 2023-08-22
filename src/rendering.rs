use image::{RgbImage, Rgb};
use crate::surface::Surface;
use crate::space3d::{Point3, ORIGIN3};

pub struct CameraConfig {
    center: Point3,
    // assuming direction = <0, 1, 0>
    // TODO: parameterize yaw, pitch, roll
    fov_h: f64,
    resolution_v: u32,
    resolution_h: u32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig {
            center: ORIGIN3,
            fov_h: 103_f64.to_radians(),
            resolution_v: 360,
            resolution_h: 480,
        }
    }
}

pub fn render<S: Surface>(camera_config: CameraConfig, _surface: S) -> RgbImage {
    let mut image = RgbImage::new(camera_config.resolution_h, camera_config.resolution_v);

    for x in 0u8..=255 {
        for y in 0u8..=255 {
            image.put_pixel(x.into(), y.into(), Rgb([0, x, y]));
        }
    }

    return image
}
