use std::rc::Rc;
use std::cmp;
use std::fmt;

use crate::expression::Expression;
use crate::expression::UndefinedError;
use crate::traits::Simplify;
use crate::types::{Power, Rational, Product, Sum, Integer, Variable};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Function {
    Sqrt(Box<Expression>),
    Cbrt(Box<Expression>),
    Log(Box<Expression>),
    Ln(Box<Expression>),
    Other(Box<str>, Box<Expression>)
}

impl Simplify for Function {
    fn simplify(self) -> Result<Expression, UndefinedError> {
        match self {
            Function::Sqrt(u) => Function::simplify_sqrt(u.simplify()?),
            Function::Cbrt(u) => Function::simplify_cbrt(u.simplify()?),
            Function::Log(u) => Function::simplify_log(u.simplify()?),
            Function::Ln(u) => Function::simplify_ln(u.simplify()?),
            Function::Other(name, u) => Ok(func!(name; u.simplify()?))
        }
    }
}

impl Function {
    pub fn new(name: impl Into<String>, arg: Expression) -> Function {
        match name.into().as_str() {
            "sqrt" => Function::Sqrt(Box::new(arg)),
            "cbrt" => Function::Cbrt(Box::new(arg)),
            "log" => Function::Log(Box::new(arg)),
            "ln" => Function::Ln(Box::new(arg)),
            name => Function::Other(Box::from(name), Box::new(arg))
        }
    }

    fn simplify_sqrt(u: Expression) -> Result<Expression, UndefinedError> {
        match pow!(u, frac!(1, 2)).simplify()? {
            Expression::Power(p) if p.exp.as_ref() == &frac!(1, 2)
                => Ok(sqrt!(*p.base)),
            v => Ok(v)
        }
    }

    fn simplify_cbrt(u: Expression) -> Result<Expression, UndefinedError> {
        match pow!(u, frac!(1, 3)).simplify()? {
            Expression::Power(p) if p.exp.as_ref() == &frac!(1, 3)
                => Ok(cbrt!(*p.base)),
            v => Ok(v)
        }
    }

    fn simplify_log(u: Expression) -> Result<Expression, UndefinedError> {
        match u {
            Expression::Power(p)
                => prod!(*p.exp, log!(*p.base)).simplify(),

            Expression::Product(p)
                => Sum::new(p.values().iter().map(|e| log!(e.clone())).collect()).simplify(),

            Expression::Rational(r)
                => sum!(log!(int!(r.num())), neg!(log!(int!(r.den())))).simplify(),

            Expression::Integer(n) if n.num() <= 0
                => Err(UndefinedError("negative or zero logarithm".to_string())),

            Expression::Integer(n) => {
                let mut power_of_ten = 1;
                let mut exp = 0;
                while power_of_ten < n.num() {
                    power_of_ten *= 10;
                    exp += 1;
                }
                if power_of_ten == n.num() {
                    Ok(int!(exp))
                }
                else {
                    Ok(ln!(n.into()))
                }
            },

            u => Ok(log!(u)),
        }
    }

    fn simplify_ln(u: Expression) -> Result<Expression, UndefinedError> {
        match u {
            Expression::Power(p)
                => prod!(*p.exp, ln!(*p.base)).simplify(),

            Expression::Product(p)
                => Sum::new(p.values().iter().map(|e| ln!(e.clone())).collect()).simplify(),

                Expression::Rational(r)
                => sum!(ln!(int!(r.num())), neg!(ln!(int!(r.den())))).simplify(),

            Expression::Variable(v) if v.as_str() == "e"
                => Ok(int!(1)),

            Expression::Integer(n) if n.num() == 1
                => Ok(int!(0)),

            Expression::Integer(n) if n.num() <= 0
                => Err(UndefinedError("negative or zero logarithm".to_string())),

            u => Ok(ln!(u)),
        }
    }
}

impl cmp::Ord for Function {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl cmp::PartialOrd for Function {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Function::Sqrt(u) => write!(f, "sqrt({})", u),
            Function::Cbrt(u) => write!(f, "cbrt({})", u),
            Function::Log(u) => write!(f, "log({})", u),
            Function::Ln(u) => write!(f, "ln({})", u),
            Function::Other(name, u) => write!(f, "{}({})", name, u)
        }
    }
}
