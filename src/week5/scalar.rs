use std::ops::*;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Scalar(pub f32);

impl Add for Scalar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Scalar(self.0 + rhs.0)
    }
}

impl Add<f32> for Scalar {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Scalar(self.0 + rhs)
    }
}

impl Add<Scalar> for f32 {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Self::Output {
        Scalar(self + rhs.0)
    }
}

impl Sub for Scalar {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Scalar(self.0 - rhs.0)
    }
}

impl Sub<f32> for Scalar {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Scalar(self.0 - rhs)
    }
}

impl Sub<Scalar> for f32 {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Self::Output {
        Scalar(self - rhs.0)
    }
}

impl Mul for Scalar {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Scalar(self.0 * rhs.0)
    }
}

impl Mul<f32> for Scalar {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Scalar(self.0 * rhs)
    }
}

impl Mul<Scalar> for f32 {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Scalar(self * rhs.0)
    }
}

impl Div for Scalar {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Scalar(self.0 / rhs.0)
    }
}

impl Div<f32> for Scalar {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Scalar(self.0 / rhs)
    }
}

impl Div<Scalar> for f32 {
    type Output = Scalar;

    fn div(self, rhs: Scalar) -> Self::Output {
        Scalar(self / rhs.0)
    }
}
