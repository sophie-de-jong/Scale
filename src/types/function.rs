use std::rc::Rc;
use std::cmp;
use std::fmt;

use crate::expression::Expression;
use crate::traits::Simplify;
use crate::types::{Power, Rational, Product, Sum, Integer, Variable};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Function {
    Sqrt(Box<Expression>),
    Cbrt(Box<Expression>),
    Log(Box<Expression>),
    Ln(Box<Expression>),
    Other(&'static str, Box<Expression>)
}

impl Simplify for Function {
    fn simplify(self) -> Option<Expression> {
        match self {
            Function::Sqrt(u) => Function::simplify_sqrt(u.simplify()?),
            Function::Cbrt(u) => Function::simplify_cbrt(u.simplify()?),
            Function::Log(u) => Function::simplify_log(u.simplify()?),
            Function::Ln(u) => Function::simplify_ln(u.simplify()?),
            Function::Other(name, u) => Some(func!(name; u.simplify()?))
        }
    }
}

impl Function {
    pub fn new(name: &'static str, arg: Expression) -> Function {
        match name {
            "sqrt" => Function::Sqrt(Box::new(arg)),
            "cbrt" => Function::Cbrt(Box::new(arg)),
            "log" => Function::Log(Box::new(arg)),
            "ln" => Function::Ln(Box::new(arg)),
            name => Function::Other(name, Box::new(arg))
        }
    }

    fn simplify_sqrt(u: Expression) -> Option<Expression> {
        match pow!(u, frac!(1, 2)).simplify()? {
            Expression::Power(p) if p.exp() == &frac!(1, 2)
                => Some(sqrt!(p.base().clone())),
            v => Some(v)
        }
    }

    fn simplify_cbrt(u: Expression) -> Option<Expression> {
        match pow!(u, frac!(1, 3)).simplify()? {
            Expression::Power(p) if p.exp() == &frac!(1, 3)
                => Some(cbrt!(p.base().clone())),
            v => Some(v)
        }
    }

    fn simplify_log(u: Expression) -> Option<Expression> {
        match u {
            Expression::Power(p)
                => prod!(p.exp().clone(), log!(p.base().clone())).simplify(),

            Expression::Product(p)
                => Sum::new(p.values().iter().map(|e| log!(e.clone())).collect()).simplify(),

            Expression::Rational(r)
                => sum!(log!(int!(r.num())), neg!(log!(int!(r.den())))).simplify(),

            Expression::Integer(n) if n.num() == 1 => Some(int!(0)),

            Expression::Integer(n) => {
                let mut power_of_ten = 1;
                let mut exp = 0;
                while power_of_ten < n.num() {
                    power_of_ten *= 10;
                    exp += 1;
                }
                if power_of_ten == n.num() {
                    Some(int!(exp))
                }
                else {
                    Some(ln!(n.into()))
                }
            },

            u => Some(log!(u)),
        }
    }

    fn simplify_ln(u: Expression) -> Option<Expression> {
        match u {
            Expression::Power(p)
                => prod!(p.exp().clone(), ln!(p.base().clone())).simplify(),

            Expression::Product(p)
                => Sum::new(p.values().iter().map(|e| ln!(e.clone())).collect()).simplify(),

                Expression::Rational(r)
                => sum!(ln!(int!(r.num())), neg!(ln!(int!(r.den())))).simplify(),

            Expression::Variable(v) if v.as_str() == "e"
                => Some(int!(1)),

            Expression::Integer(n) if n.num() == 0
                => Some(int!(0)),

            u => Some(ln!(u)),
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
