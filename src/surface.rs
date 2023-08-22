use crate::space3d::{Point3, ORIGIN3};

pub trait Surface { }

pub struct VoidSurface { }
impl Surface for VoidSurface { }

pub const VOID_SURFACE: VoidSurface = VoidSurface { };


pub struct Sphere {
    center: Point3,
    radius: f64,
}
impl Surface for Sphere { }

pub const UNIT_SPHERE: Sphere = Sphere { center: ORIGIN3, radius: 1.0 };


pub struct Triangle {
    a: Point3,
    b: Point3,
    c: Point3,
}
impl Surface for Triangle { }
