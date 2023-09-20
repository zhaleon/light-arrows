use glam::DVec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

pub struct HitInfo {
    pub contact_time: f64,
    pub contact_point: DVec3,
    pub contact_normal: DVec3,
    pub material: DVec3,
}
