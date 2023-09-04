use glam::DVec3;
use image::Rgb;
use crate::{ray::{Ray, HitInfo}, material::Material};

pub trait Surface {
    fn intersect(self, ray: Ray) -> Option<HitInfo>;
}

pub struct VoidSurface { }
impl Surface for VoidSurface {
    fn intersect (self, _: Ray) -> Option<HitInfo> { None }
}

pub const VOID_SURFACE: VoidSurface = VoidSurface { };

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
    pub material: Material,
}
impl Surface for Sphere {
    fn intersect (self, ray: Ray) -> Option<HitInfo> {
        // intersect the line t*p with the sphere centered at (c - o)
        let p = ray.direction;
        let o = ray.origin;
        let c = self.center;
        let r = self.radius;
        let delta = DVec3::dot(p, o - c).powi(2) - p.length_squared() * ((o - c).length_squared() - r * r);
        if delta < 0.0 {
            return None
        }
        let t1 = (- DVec3::dot(p, o - c) - delta.sqrt()) / p.length_squared();
        let t2 = (- DVec3::dot(p, o - c) + delta.sqrt()) / p.length_squared();
        if t1 < 0.0 && t2 < 0.0 {
            return None
        } else {
            let contact_time = if t1 < 0.0 { t2 } else { t1 };
            let contact_point = ray.origin + contact_time * ray.direction;
            Some(HitInfo{
                contact_time,
                contact_point,
                contact_normal: contact_point - self.center,
                material: self.material.color,
            })
        }
    }
}

pub const UNIT_SPHERE: Sphere = Sphere {
    center: DVec3::ZERO,
    radius: 1.0,
    material: Material{
        color: Rgb([255, 255, 255])
    },
};

/*
pub struct Triangle {
    pub a: Point3,
    pub b: Point3,
    pub c: Point3,
}
impl Surface for Triangle { }
*/
