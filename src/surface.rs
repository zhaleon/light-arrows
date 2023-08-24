use glam::DVec3;
use crate::ray::Ray;

pub trait Surface {
    fn intersect(self, ray: Ray) -> Option<DVec3>;
}

pub struct VoidSurface { }
impl Surface for VoidSurface {
    fn intersect (self, _: Ray) -> Option<DVec3> { None }
}

pub const VOID_SURFACE: VoidSurface = VoidSurface { };


pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
}
impl Surface for &Sphere {
    fn intersect (self, ray: Ray) -> Option<DVec3> {
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
            None
        } else if t1 < 0.0 {
            Some(ray.origin + t2 * ray.direction)
        } else {
            Some(ray.origin + t1 * ray.direction)
        }
    }
}

pub const UNIT_SPHERE: Sphere = Sphere { center: DVec3::ZERO, radius: 1.0 };

/*
pub struct Triangle {
    pub a: Point3,
    pub b: Point3,
    pub c: Point3,
}
impl Surface for Triangle { }
*/
