use std::f64::consts::E;
use std::f64::EPSILON;
use std::fmt::{Debug, Display};
use std::ops::{Add, BitXor, Div, Mul, Sub};

use crate::week5::scalar::Scalar;

#[derive(Clone, Debug)]
pub struct DerivableFunction(pub String, pub fn(Expression) -> Expression);
impl PartialEq for DerivableFunction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

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
    DerivableFunctionExpression(DerivableFunction, Box<Expression>),
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
            (
                Self::DerivableFunctionExpression(fun1, inside1),
                Self::DerivableFunctionExpression(fun2, inside2),
            ) => fun1 == fun2 && inside1 == inside2,
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
                Expression::DerivableFunctionExpression(name, inside) => {
                    Expression::DerivableFunctionExpression(
                        name.clone(),
                        Box::new(inside.simplified()),
                    )
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
                if zero.abs() <= EPSILON =>
            {
                *v
            }

            Expression::Subtract(
                box Expression::Constant(Scalar(lhs)),
                box Expression::Constant(Scalar(rhs)),
            ) => Expression::Constant(Scalar(lhs - rhs)),
            Expression::Subtract(v, box Expression::Constant(Scalar(zero)))
                if zero.abs() <= EPSILON =>
            {
                *v
            }

            Expression::Multiply(box Expression::Constant(Scalar(zero)), _)
            | Expression::Multiply(_, box Expression::Constant(Scalar(zero)))
                if zero.abs() <= EPSILON =>
            {
                Expression::Constant(Scalar(0.))
            }
            Expression::Multiply(box Expression::Constant(Scalar(one)), other)
            | Expression::Multiply(other, box Expression::Constant(Scalar(one)))
                if (1. - one).abs() <= EPSILON =>
            {
                *other
            }
            Expression::Multiply(
                box Expression::Constant(Scalar(lhs)),
                box Expression::Constant(Scalar(rhs)),
            ) => Expression::Constant(Scalar(lhs * rhs)),
            // Expression::Multiply(_, _) => todo!(),
            Expression::Divide(other, box Expression::Constant(Scalar(one)))
                if (1. - one).abs() <= EPSILON =>
            {
                *other
            }

            Expression::Divide(
                box Expression::Constant(Scalar(lhs)),
                box Expression::Constant(Scalar(rhs)),
            ) => Expression::Constant(Scalar(lhs / rhs)),
            // Expression::Divide(_, _) => todo!(),
            
            // 1^a
            Expression::Exponentiate(box Expression::Constant(Scalar(one)), _)
                if (1. - one).abs() <= EPSILON =>
            {
                Expression::Constant(Scalar(1.))
            }

            // a^1
            Expression::Exponentiate(other, box Expression::Constant(Scalar(one)))
                if (1. - one).abs() <= EPSILON =>
            {
                *other
            }

            // 0^a
            Expression::Exponentiate(box Expression::Constant(Scalar(zero)), _)
                if zero.abs() <= EPSILON =>
            {
                Expression::Constant(Scalar(0.))
            }

            // a^0
            Expression::Exponentiate(_, box Expression::Constant(Scalar(zero)))
                if zero.abs() <= EPSILON =>
            {
                Expression::Constant(Scalar(1.))
            }

            // a^b
            Expression::Exponentiate(
                box Expression::Constant(Scalar(base)),
                box Expression::Constant(Scalar(power)),
            ) => Expression::Constant(Scalar(base.powf(power))),
            // Expression::Exponentiate(_, _) => todo!(),

            
            Expression::Logarithm(
                box Expression::Constant(Scalar(base)),
                box Expression::Constant(Scalar(inside)),
            ) => Expression::Constant(Scalar(inside.log(base))),
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
            Expression::Multiply(box Self::Constant(Scalar(neg_one)), other)
            | Expression::Multiply(other, box Self::Constant(Scalar(neg_one)))
                if (neg_one + 1.).abs() <= EPSILON =>
            {
                write!(f, "-({})", other)
            }
            Expression::Multiply(lhs, rhs) => write!(f, "({} * {})", lhs, rhs),
            Expression::Divide(lhs, rhs) => write!(f, "({} / {})", lhs, rhs),
            Expression::Exponentiate(base, power) => write!(f, "({} ^ {})", base, power),
            Expression::Logarithm(box Expression::Constant(Scalar(e)), inside)
                if E - e <= EPSILON =>
            {
                write!(f, "ln({})", inside)
            }
            Expression::Logarithm(base, inside) => write!(f, "log_({})({})", base, inside),
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::Constant(value) => write!(f, "{}", value.0),
            Expression::DerivableFunctionExpression(DerivableFunction(name, _), inside) => {
                write!(f, "{}({})", name, inside)
            }
        }
    }
}

pub mod functions {
    use crate::week5::scalar::Scalar;

    use super::{DerivableFunction, Expression};

    pub fn sin(inside: &Expression) -> Expression {
        Expression::DerivableFunctionExpression(
            DerivableFunction("sin".to_string(), |inside| cos(&inside)),
            Box::new(inside.clone()),
        )
    }

    pub fn cos(inside: &Expression) -> Expression {
        Expression::DerivableFunctionExpression(
            DerivableFunction("cos".to_string(), |inside| {
                Expression::Multiply(
                    Box::new(Expression::Constant(Scalar(-1.))),
                    Box::new(sin(&inside)),
                )
            }),
            Box::new(inside.clone()),
        )
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

        let x = Expression::Variable('x');
        let zero = Expression::Constant(Scalar(0.));
        let seven = Expression::Constant(Scalar(7.));
        let eight = Expression::Constant(Scalar(8.));

        assert_eq!(((x ^ zero) + seven).simplified(), eight);

        let x1 = Expression::Variable('x');
        let x2 = Expression::Variable('x');
        let one = Expression::Constant(Scalar(1.));

        assert_eq!((x1 ^ one).simplified(), x2);
    }

    #[test]
    fn display() {
        let ex: Expression = Expression::Constant(Scalar(1.))
            / (Expression::Constant(Scalar(1.))
                - (Expression::Variable('x') ^ Expression::Constant(Scalar(2.))));
        println!("f(x)={}", ex);
    }

    #[test]
    fn sin() {
        let ex: Expression = Expression::Constant(Scalar(1.))
            / (Expression::Constant(Scalar(1.))
                - (Expression::Variable('x') ^ Expression::Constant(Scalar(2.))));

        println!("f(x)={}", super::functions::sin(&ex));
    }
}
