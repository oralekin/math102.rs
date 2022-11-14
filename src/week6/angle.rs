use std::f32::consts::PI;

pub trait Angle: Into<Degrees> + Into<Radians> + Copy {
    fn to_unit_circle(self) -> Self;

    fn is_eq<T: Angle>(&self, other: T) -> bool;
    fn is_equivalent<T: Angle>(&self, other: T) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub struct Radians(pub f32);

impl From<Degrees> for Radians {
    fn from(deg: Degrees) -> Radians {
        Radians(deg.0.to_radians())
    }
}

impl Angle for Radians {
    fn to_unit_circle(self) -> Self {
        Radians(self.0 % (2.0 * PI))
    }

    fn is_eq<T: Angle>(&self, other: T) -> bool {
        self.0 == Into::<Radians>::into(other).0
    }

    fn is_equivalent<T: Angle>(&self, other: T) -> bool {
        ((self.to_unit_circle()).0) == (Into::<Radians>::into(other).to_unit_circle().0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Degrees(pub f32);

impl From<Radians> for Degrees {
    fn from(deg: Radians) -> Degrees {
        Degrees(deg.0.to_degrees())
    }
}

impl Angle for Degrees {
    fn to_unit_circle(self) -> Self {
        todo!()
    }

    fn is_eq<T>(&self, other: T) -> bool
    where
        T: Angle,
    {
        self.0 == Into::<Degrees>::into(other.clone()).0
    }

    fn is_equivalent<T: Angle>(&self, other: T) -> bool {
        Into::<Radians>::into(other.clone()).is_equivalent(other)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::{Angle, Degrees, Radians};

    #[test]
    fn equivalence() {
        assert_eq!(true, Degrees(240.0).is_equivalent(Degrees(-60.0)));
        assert_eq!(true, Radians(1.0).is_equivalent(Radians((2.0 * PI) + 1.0)));
    }
}
