use glam::DVec3;

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

pub struct HitInfo {
    pub contact_point: DVec3,
    pub direction: DVec3,
}
