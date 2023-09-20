use glam::DVec3;

pub fn sky_light_in_direction(dir: DVec3) -> DVec3 {
    if dir.z < 0.0 {
        return DVec3::new(120.0, 170.0, 100.0) / 255.0;
    }

    let sun_direction = DVec3::new(-1.0, 1.0, 1.0).normalize();
    let intensity = dir.normalize_or_zero().dot(sun_direction).clamp(0.0, 1.0);

    // maybe make this gradient better
    let sun_color = DVec3::new(255.0, 255.0, 255.0) / 255.0;
    let horizon_color = DVec3::new(160.0, 160.0, 255.0) / 255.0;
    let color_f = sun_color * intensity + horizon_color * (1.0 - intensity);

    return color_f
}
