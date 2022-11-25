// alternatively, cross product is defined as the determinant of a matrix where the first row is the unit vectors in order and the rest of the rows are the vectors being operated on.

use std::ops::{Add, Mul, Sub};

use crate::week5::{scalar::Scalar, vector};

pub fn cross(vectors: Vec<vector::Vector>) -> Value {
    let mut matrix: Vec<Value> = vec![];

    matrix.append(&mut units(vectors.len() + 1));
    matrix.append(
        &mut vectors
            .clone()
            .into_iter()
            .flat_map(|v| v.0)
            .map(|v| Value::Scalar(Scalar(v)))
            .collect(),
    );

    determinant(matrix)
}

pub fn units(dimension: usize) -> Vec<Value> {
    (0..dimension)
        .map(|i| {
            let mut v = vec![0.; dimension];
            v[i] = 1.;
            Value::Vector(vector::Vector(v))
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Vector(vector::Vector),
    Scalar(Scalar),
}

impl Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        match self {
            Value::Vector(inner) => match rhs {
                Value::Vector(rhs_inner) => Value::Vector(inner + rhs_inner),
                Value::Scalar(_) => panic!(),
            },
            Value::Scalar(inner) => match rhs {
                Value::Scalar(rhs_inner) => Value::Scalar(inner + rhs_inner),
                Value::Vector(_) => panic!(),
            },
        }
    }
}

impl Sub<Value> for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        match self {
            Value::Vector(inner) => match rhs {
                Value::Vector(rhs_inner) => Value::Vector(inner - rhs_inner),
                Value::Scalar(_) => panic!(),
            },
            Value::Scalar(inner) => match rhs {
                Value::Scalar(rhs_inner) => Value::Scalar(inner - rhs_inner),
                Value::Vector(_) => panic!(),
            },
        }
    }
}

impl Mul<Value> for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        match self {
            Value::Vector(inner) => match rhs {
                Value::Vector(rhs_inner) => Value::Scalar(inner * rhs_inner),
                Value::Scalar(rhs_inner) => Value::Vector(inner * rhs_inner),
            },
            Value::Scalar(inner) => match rhs {
                Value::Scalar(rhs_inner) => Value::Scalar(inner * rhs_inner),
                Value::Vector(rhs_inner) => Value::Vector(rhs_inner * inner),
            },
        }
    }
}

pub type SquareMatrix = Vec<Value>;

pub fn determinant(matrix: SquareMatrix) -> Value {
    println!("{:?}", matrix);

    if matrix.len() == 1 {
        matrix.get(0).unwrap().clone()
    } else {
        let size = (matrix.len() as f64).sqrt() as usize;

        (0..size)
            .map(|col| {
                determinant(
                    matrix
                        .clone()
                        .into_iter()
                        .enumerate()
                        .filter(|(i, _)| *i >= size && i % size != col)
                        .map(|(_, v)| v)
                        .collect(),
                ) * (matrix.get(col).unwrap().clone())
            })
            .enumerate()
            .reduce(|(_, acc), (i, v)| (0, if let 0 = i % 2 { acc + v } else { acc - v }))
            .map(|(_, v)| v)
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::week5::{
        scalar::Scalar,
        vector::{
            unit::three_d::{i, j, k},
            Vector,
        },
    };

    use super::{cross, determinant, Value};

    #[test]
    fn basecase() {
        assert_eq!(
            determinant(vec![Value::Scalar(Scalar(1.))]),
            Value::Scalar(Scalar(1.))
        )
    }

    #[test]
    fn units() {
        assert_eq!(cross(vec![i(), j()]), Value::Vector(k()));
        assert_eq!(cross(vec![j(), k()]), Value::Vector(i()));
        assert_eq!(cross(vec![k(), i()]), Value::Vector(j()));
    }

    #[test]
    fn threedimensional() {
        assert_eq!(
            cross(vec![
                Vector(vec![2.0, 1.0, 3.0]),
                Vector(vec![-1.0, 2.0, 2.0])
            ]),
            Value::Vector(Vector(vec![2.0, 1.0, 3.0]) ^ Vector(vec![-1.0, 2.0, 2.0]))
        )
    }
}
