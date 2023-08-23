use crate::space3d::{Point3, ORIGIN3, Ray};

pub trait Surface {
    fn intersect(self, ray: Ray) -> Option<Point3>;
}

pub struct VoidSurface { }
impl Surface for VoidSurface {
    fn intersect (self, _: Ray) -> Option<Point3> { None }
}

pub const VOID_SURFACE: VoidSurface = VoidSurface { };


pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}
impl Surface for &Sphere {
    fn intersect (self, ray: Ray) -> Option<Point3> {
        // intersect the line t*p with the sphere centered at (c - o)
        let p = ray.direction;
        let c = self.center - ray.origin;
        let r = self.radius;
        let delta = (p * c).powi(2) - (p * p) * (c * c - r * r);
        if delta < 0.0 {
            return None
        }
        let t1 = (p * c - delta.sqrt()) / (p * p);
        let t2 = (p * c + delta.sqrt()) / (p * p);
        if t1 < 0.0 && t2 < 0.0 {
            None
        } else if t1 < 0.0 {
            Some(ray.origin + t2 * ray.direction)
        } else {
            Some(ray.origin + t1 * ray.direction)
        }
    }
}

pub const UNIT_SPHERE: Sphere = Sphere { center: ORIGIN3, radius: 1.0 };

/*
pub struct Triangle {
    pub a: Point3,
    pub b: Point3,
    pub c: Point3,
}
impl Surface for Triangle { }
*/
