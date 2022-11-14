use std::ops::BitXor;

use crate::week5::vector::Vector;

impl Vector {
    fn crossed(&self, other: &Self) -> Result<Self, ()> {
        if self.0.len() != 3 || other.0.len() != 3 { Err(()) }
        else { 
            let u = &self.0;
            let v = &other.0;
            Ok(Vector(vec![
                ((u[1] * v[2]) - (u[2] * v[1])),
              - ((u[0] * v[2]) - (u[2] * v[0])),
                ((u[0] * v[1]) - (u[1] * v[0])),
        ])) }
    } 
}

impl BitXor<Vector> for Vector {
    type Output = Self;

    fn bitxor(self, rhs: Vector) -> Self::Output {
        self.crossed(&rhs).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::week5::vector::{unit::three_d::{i, j, k}, Vector};

    #[test]
    fn units() {
        assert_eq!(i()^j(), k());
        assert_eq!(j()^k(), i());
        assert_eq!(k()^i(), j());
    }

    #[test]
    fn example() {
        assert_eq!(
            Vector(vec![2.0,1.0,3.0]) ^ Vector(vec![-1.0, 2.0, 2.0]), 
            Vector(vec![-4.0, -7.0, 5.0])
        )
    }
}