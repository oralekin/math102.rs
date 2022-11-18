use std::ops::{Add, Sub};

use crate::week5::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3(pub f64, pub f64, pub f64);

impl Vector {
    pub fn from_points(p1: Point3, p2: Point3) -> Vector {
        &p2 - &p1
    }
}

impl Point3 {
    pub const ZERO: Point3 = Point3(0.0, 0.0, 0.0);
}

impl Sub<Point3> for Point3 {
    type Output = Vector;

    fn sub(self, rhs: Point3) -> Self::Output {
        Vector(vec![rhs.0 - self.0, rhs.1 - self.1, rhs.2 - self.2])
    }
}

impl Sub<&Point3> for Point3 {
    type Output = Vector;

    fn sub(self, rhs: &Point3) -> Self::Output {
        Vector(vec![rhs.0 - self.0, rhs.1 - self.1, rhs.2 - self.2])
    }
}

impl Sub<&Point3> for &Point3 {
    type Output = Vector;

    fn sub(self, rhs: &Point3) -> Self::Output {
        Vector(vec![rhs.0 - self.0, rhs.1 - self.1, rhs.2 - self.2])
    }
}

impl Add<Vector> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vector) -> Self::Output {
        if rhs.0.len() != 3 {
            panic!()
        };

        Point3(self.0 + rhs.0[0], self.1 + rhs.0[1], self.2 + rhs.0[2])
    }
}
