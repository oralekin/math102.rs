use std::{ops::*, vec};

use crate::week5::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector(pub vec::Vec<f64>);

impl Vector {
    pub fn magnitude_squared(&self) -> Scalar {
        Scalar(self.0.iter().map(|x| x * x).sum::<f64>())
    }

    pub fn magnitude(&self) -> Scalar {
        Scalar(self.magnitude_squared().0.sqrt())
    }

    pub fn unit_in_direction(&self) -> Vector {
        let magnitude = self.magnitude();
        Vector(self.0.iter().map(|x| x / magnitude.0).collect())
    }

    pub fn into_unit_in_direction(self) -> Vector {
        let magnitude = self.magnitude();
        Vector(self.0.into_iter().map(|x| x / magnitude.0).collect())
    }

    pub fn invert(&mut self) {
        self.0.iter_mut().for_each(|x| *x *= -1.0);
    }

    pub fn as_inverse(self) -> Vector {
        Vector(self.0.into_iter().map(|x| -x).collect())
    }

    pub fn inverted(&self) -> Vector {
        Vector(self.0.iter().map(|x| -x).collect())
    }

    pub fn added(&self, other: &Self) -> Result<Self, &str> {
        if self.0.len() != other.0.len() {
            return Err("Mismatched dimensions");
        }
        Ok(Vector(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(x1, x2)| x1 + x2)
                .collect(),
        ))
    }

    pub fn subtracted(&self, other: &Self) -> Result<Self, &str> {
        self.added(&other.inverted())
    }

    pub fn multiply(&mut self, scalar: Scalar) {
        self.0.iter_mut().for_each(|x| *x *= scalar.0);
    }

    pub fn as_multiplied(self, scalar: Scalar) -> Vector {
        Vector(self.0.into_iter().map(|x| x * scalar.0).collect())
    }

    pub fn multiplied(&self, scalar: Scalar) -> Vector {
        Vector(self.0.iter().map(|x| x * scalar.0).collect())
    }
}

// ========== vector - vector operations ==========

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        self.added(&rhs).unwrap()
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subtracted(&rhs).unwrap()
    }
}

// ========== vector - scalar operations ==========

impl Mul<Scalar> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Scalar) -> Self::Output {
        self.multiplied(rhs)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        self.multiplied(Scalar(rhs))
    }
}

impl Div<Scalar> for Vector {
    type Output = Vector;

    fn div(self, rhs: Scalar) -> Self::Output {
        self * (1.0 / rhs.0)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

pub mod unit {
    pub mod two_d {
        use crate::week5::vector::Vector;

        pub fn i() -> Vector {
            Vector(vec![1.0, 0.0])
        }
        pub fn j() -> Vector {
            Vector(vec![0.0, 1.0])
        }
    }
    pub mod three_d {
        use crate::week5::vector::Vector;

        pub fn i() -> Vector {
            Vector(vec![1.0, 0.0, 0.0])
        }
        pub fn j() -> Vector {
            Vector(vec![0.0, 1.0, 0.0])
        }
        pub fn k() -> Vector {
            Vector(vec![0.0, 0.0, 1.0])
        }
    }
}
