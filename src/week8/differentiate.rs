use crate::week5::scalar::Scalar;

use super::expression::Expression;

#[derive(Debug)]
struct BadDifferentiationError;
trait Differentiate {
    fn differentiate(&self, wrt: &Expression) -> Result<Expression, BadDifferentiationError>;
}

impl Differentiate for Expression {
    fn differentiate(&self, wrt: &Expression) -> Result<Expression, BadDifferentiationError> {
        if let Expression::Variable(wrt_name) = wrt {
            Ok(match self {
                Expression::Add(lhs, rhs) => lhs.differentiate(wrt)? + rhs.differentiate(wrt)?,
                Expression::Subtract(lhs, rhs) => 
                    lhs.differentiate(wrt)? - rhs.differentiate(wrt)?,
                
                Expression::Multiply(lhs, rhs) =>
                    (lhs.differentiate(wrt)? * *rhs.clone())
                    + (rhs.differentiate(wrt)? * *lhs.clone()),
                
                Expression::Divide(lhs, rhs) => 
                    (lhs.differentiate(wrt)? * *rhs.clone())
                    + (rhs.differentiate(wrt)? * *lhs.clone()),
                
                Expression::Exponentiate(base, power) => 
                      *power.clone()
                    * (*base.clone() ^ (
                        *power.clone() - Expression::Constant(Scalar(1.))
                      ))
                    * base.differentiate(wrt)?,

                Expression::Logaritm(base, inside) => (Expression::Constant(Scalar(1.)) / *inside.clone()) * base.differentiate(wrt)?,
                Expression::Variable(name) => {
                    if wrt_name == name {
                        Expression::Constant(Scalar(1.))
                    } else {
                        Expression::Constant(Scalar(0.))
                    }
                }
                Expression::Constant(Scalar(_)) => Expression::Constant(Scalar(0.)),
            }.simplified())
        } else {
            Err(BadDifferentiationError)
        }
    }
}


#[cfg(test)]
mod test {
    use crate::{week8::{expression::Expression, differentiate::Differentiate}, week5::scalar::Scalar};

    #[test]
    fn constant_coefficient() {
        assert_eq!(
            Expression::Multiply(
                Box::new(Expression::Constant(Scalar(5.))), 
                Box::new(Expression::Variable('x'))
            ).differentiate(&Expression::Variable('x')).unwrap(),
            Expression::Constant(Scalar(5.))
        );
    }

    #[test]
    fn power_rule() {
        assert_eq!(
            Expression::Exponentiate(
                Box::new(Expression::Variable('x')),
                Box::new(Expression::Constant(Scalar(5.))), 
            ).differentiate(&Expression::Variable('x')).unwrap(),
            (Expression::Constant(Scalar(5.)) * (Expression::Variable('x') ^ Expression::Constant(Scalar(4.)))).simplified()
        );
    }
} 