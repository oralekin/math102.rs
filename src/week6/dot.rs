use std::ops::{Shr, Shl, Mul};

use crate::week5::{scalar::Scalar, vector::Vector};

use super::angle::Radians;

impl Vector {
    pub fn dotted(&self, other: &Self) -> Scalar {
        Scalar(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(x1, x2)| x1 * x2)
                .sum(),
        )
    }

    pub fn projected_on(&self, base: &Self) -> Self {
        base.multiplied(self.dotted(base) / base.magnitude_squared())
    }

    pub fn angle_between(&self, other: &Vector) -> Radians {
        Radians(
            (self.dotted(other) / (self.magnitude() * other.magnitude()))
                .0
                .acos(),
        )
    }

}

// dot product
impl Mul<Vector> for Vector {
    type Output = Scalar;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dotted(&rhs)
    }
}

// projection

impl Shl<Vector> for Vector {
    type Output = Vector;

    fn shl(self, rhs: Vector) -> Self::Output {
        rhs.projected_on(&self)
    }
}

impl Shr<Vector> for Vector {
    type Output = Vector;

    fn shr(self, rhs: Vector) -> Self::Output {
        self.projected_on(&rhs)
    }
}

