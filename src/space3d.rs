use std::ops::{Mul, Add, Sub, Neg};

#[derive(Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub const ORIGIN3: Point3 = Point3 { x: 0.0, y: 0.0, z: 0.0 };

impl Add for Point3 {
    type Output = Point3;
    fn add(self, rhs: Point3) -> Point3 {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Neg for Point3 {
    type Output = Point3;
    fn neg(self) -> Point3 {
        Point3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Point3 {
    type Output = Point3;
    fn sub(self, rhs: Point3) -> Point3 {
        Point3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Point3 {
    type Output = f64;

    fn mul(self, rhs: Point3) -> f64 {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z
    }
}

impl Mul<f64> for Point3 {
    type Output = Point3;

    fn mul(self, rhs: f64) -> Point3 {
        Point3 {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl Mul<Point3> for f64 {
    type Output = Point3;

    fn mul(self, rhs: Point3) -> Point3 {
        Point3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

pub struct Ray {
    pub origin: Point3,
    pub direction: Point3,
}
