use glam::DVec3;
use image::Rgb;

pub fn sky_color_in_direction(dir: DVec3) -> Rgb<u8> {
    if dir.z < 0.0 {
        return Rgb([120, 170, 100])
    }

    let sun_direction = DVec3::new(-1.0, 1.0, 1.0).normalize();
    let intensity = dir.normalize_or_zero().dot(sun_direction).clamp(0.0, 1.0);

    // maybe make this gradient better
    let sun_color = DVec3::new(255.0, 255.0, 255.0);
    let horizon_color = DVec3::new(160.0, 160.0, 255.0);
    let color_f = sun_color * intensity + horizon_color * (1.0 - intensity);
    Rgb(color_f.to_array().map(|f| {f as u8}))
}
