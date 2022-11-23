use std::f64::EPSILON;
use std::f64::consts::E;
use std::fmt::{Debug, Display};
use std::ops::{Add, BitXor, Div, Mul, Sub};

use crate::week5::scalar::Scalar;

#[derive(Debug, Clone)]
pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Exponentiate(Box<Expression>, Box<Expression>),
    Logarithm(Box<Expression>, Box<Expression>),
    Variable(char),
    Constant(Scalar),
}

impl Add for Expression {
    type Output = Expression;

    fn add(self, rhs: Self) -> Self::Output {
        Expression::Add(Box::new(self), Box::new(rhs))
    }
}

impl Sub for Expression {
    type Output = Expression;

    fn sub(self, rhs: Self) -> Self::Output {
        Expression::Subtract(Box::new(self), Box::new(rhs))
    }
}

impl Mul for Expression {
    type Output = Expression;

    fn mul(self, rhs: Self) -> Self::Output {
        Expression::Multiply(Box::new(self), Box::new(rhs))
    }
}

impl Div for Expression {
    type Output = Expression;

    fn div(self, rhs: Self) -> Self::Output {
        Expression::Divide(Box::new(self), Box::new(rhs))
    }
}

impl BitXor for Expression {
    type Output = Expression;

    fn bitxor(self, rhs: Self) -> Self::Output {
        println!("{:?} to the {:?}", self, rhs);
        Expression::Exponentiate(Box::new(self), Box::new(rhs))
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Add(l0, l1), Self::Add(r0, r1)) => {
                (l0 == r0 && l1 == r1) || (l0 == r1 && l1 == r0)
            }
            (Self::Subtract(l0, l1), Self::Subtract(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Multiply(l0, l1), Self::Multiply(r0, r1)) => {
                (l0 == r0 && l1 == r1) || (l0 == r1 && l1 == r0)
            }
            (Self::Divide(l0, l1), Self::Divide(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Exponentiate(l0, l1), Self::Exponentiate(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Logarithm(l0, l1), Self::Logarithm(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Variable(l0), Self::Variable(r0)) => l0 == r0,
            (Self::Constant(l0), Self::Constant(r0)) => l0 == r0,
            (_, _) => false,
        }
    }
}

impl Expression {
    pub fn simplified(&self) -> Expression {
        match {
            match self {
                Expression::Add(lhs, rhs) => lhs.simplified() + rhs.simplified(),
                Expression::Subtract(lhs, rhs) => lhs.simplified() - rhs.simplified(),
                Expression::Multiply(lhs, rhs) => lhs.simplified() * rhs.simplified(),
                Expression::Divide(lhs, rhs) => lhs.simplified() / rhs.simplified(),
                Expression::Exponentiate(lhs, rhs) => lhs.simplified() ^ rhs.simplified(),
                Expression::Logarithm(lhs, rhs) => {
                    Expression::Logarithm(Box::new(lhs.simplified()), Box::new(rhs.simplified()))
                }
                other => other.clone(),
            }
        } {
            Expression::Add(
                box Expression::Constant(Scalar(lhs)),
                box Expression::Constant(Scalar(rhs)),
            ) => Expression::Constant(Scalar(lhs + rhs)),
            Expression::Add(box Expression::Constant(Scalar(zero)), v)
            | Expression::Add(v, box Expression::Constant(Scalar(zero)))
                if zero.abs() <= f64::EPSILON =>
            {
                *v
            }

            Expression::Subtract(
                box Expression::Constant(Scalar(lhs)),
                box Expression::Constant(Scalar(rhs)),
            ) => Expression::Constant(Scalar(lhs - rhs)),
            Expression::Subtract(v, box Expression::Constant(Scalar(zero)))
                if zero.abs() <= f64::EPSILON =>
            {
                *v
            }

            Expression::Multiply(box Expression::Constant(Scalar(zero)), _)
            | Expression::Multiply(_, box Expression::Constant(Scalar(zero)))
                if zero.abs() <= f64::EPSILON =>
            {
                Expression::Constant(Scalar(0.))
            }
            Expression::Multiply(box Expression::Constant(Scalar(one)), other)
            | Expression::Multiply(other, box Expression::Constant(Scalar(one)))
                if (1. - one).abs() <= f64::EPSILON =>
            {
                *other
            }
            Expression::Multiply(
                box Expression::Constant(Scalar(lhs)),
                box Expression::Constant(Scalar(rhs)),
            ) => Expression::Constant(Scalar(lhs * rhs)),
            // Expression::Multiply(_, _) => todo!(),

            // Expression::Divide(_, _) => todo!(),
            // Expression::Exponentiate(_, _) => todo!(),
            // Expression::Logarithm(_, _) => todo!(),
            other => other,
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Add(lhs, rhs) => write!(f, "({} + {})", lhs, rhs),
            Expression::Subtract(lhs, rhs) => write!(f, "({} - {})", lhs, rhs),
            Expression::Multiply(lhs, rhs) => write!(f, "({} * {})", lhs, rhs),
            Expression::Divide(lhs, rhs) => write!(f, "({} - {})", lhs, rhs),
            Expression::Exponentiate(base, power) => write!(f, "({} ^ {})", base, power),
            Expression::Logarithm(box Expression::Constant(Scalar (e)), inside) if E-e <= EPSILON => write!(f, "ln({})", inside),
            Expression::Logarithm(base, inside) => write!(f, "log_({})({})", base, inside),
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::Constant(value) => write!(f, "{}", value.0),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::week5::scalar::Scalar;

    use super::Expression;

    fn scalar(v: f64) -> Expression {
        Expression::Constant(Scalar(v))
    }

    #[test]
    fn eq() {
        assert_eq!(
            Expression::Add(Box::new(Expression::Variable('x')), Box::new(scalar(5.))),
            Expression::Add(Box::new(Expression::Variable('x')), Box::new(scalar(5.))),
        );

        assert_eq!(
            Expression::Add(Box::new(scalar(5.)), Box::new(Expression::Variable('x'))),
            Expression::Add(Box::new(Expression::Variable('x')), Box::new(scalar(5.))),
        );

        assert_ne!(
            Expression::Divide(Box::new(scalar(5.)), Box::new(Expression::Variable('x'))),
            Expression::Divide(Box::new(Expression::Variable('x')), Box::new(scalar(5.))),
        );
    }

    #[test]
    fn simplify() {
        let three = Expression::Constant(Scalar(3.));
        let four = Expression::Constant(Scalar(4.));
        let seven = Expression::Constant(Scalar(7.));

        // println!("{:?}", (three + four).simplified());
        assert_eq!((three + four).simplified(), seven);

        let x = Expression::Variable('x');
        let zero = Expression::Constant(Scalar(0.));
        let seven1 = Expression::Constant(Scalar(7.));
        let seven2 = Expression::Constant(Scalar(7.));

        assert_eq!(((x * zero) + seven1).simplified(), seven2);
    }

    #[test]
    fn display() {
        let ex: Expression = 
            Expression::Constant(Scalar(1.)) 
            / (
                Expression::Constant(Scalar(1.)) - (
                    Expression::Variable('x') ^ Expression::Constant(Scalar(2.))
                )
            );
        println!("f(x)={}", ex);
    }

}
