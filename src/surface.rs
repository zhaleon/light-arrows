use crate::space3d::{Point3, ORIGIN3};

pub trait Surface { }

pub struct VoidSurface { }
impl Surface for VoidSurface { }

pub const VOID_SURFACE: VoidSurface = VoidSurface { };


pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}
impl Surface for Sphere { }

pub const UNIT_SPHERE: Sphere = Sphere { center: ORIGIN3, radius: 1.0 };


pub struct Triangle {
    pub a: Point3,
    pub b: Point3,
    pub c: Point3,
}
impl Surface for Triangle { }
