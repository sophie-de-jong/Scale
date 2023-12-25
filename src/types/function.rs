use std::rc::Rc;
use std::cmp;

use crate::expression::Expression;
use crate::traits::Simplify;
use crate::types::{Power, Rational, Product, Sum, Integer, Variable};

#[derive(Debug, PartialEq, Eq, Clone, strum::Display)]
pub enum Function {
    Sqrt(Box<Expression>),
    Cbrt(Box<Expression>),
    Log(Box<Expression>),
    Ln(Box<Expression>),
}

impl Simplify for Function {
    fn simplify(self) -> Option<Expression> {
        match self {
            Function::Sqrt(u) => Function::simplify_sqrt(u.simplify()?),
            Function::Cbrt(u) => Function::simplify_cbrt(u.simplify()?),
            Function::Log(u) => Function::simplify_log(u.simplify()?),
            Function::Ln(u) => Function::simplify_ln(u.simplify()?),
        }
    }
}

impl Function {
    fn simplify_sqrt(u: Expression) -> Option<Expression> {
        match pow!(u, frac!(1, 2)).simplify()? {
            Expression::Power(Power(v, r)) if *r == frac!(1, 2)
                => Some(sqrt!(*v)),
            v => Some(v)
        }
    }

    fn simplify_cbrt(u: Expression) -> Option<Expression> {
        match pow!(u, frac!(1, 3)).simplify()? {
            Expression::Power(Power(v, r)) if *r == frac!(1, 3)
                => Some(cbrt!(*v)),
            v => Some(v)
        }
    }

    fn simplify_log(u: Expression) -> Option<Expression> {
        match u {
            Expression::Power(p)
                => prod!(*p.1, log!(*p.0)).simplify(),
            Expression::Product(p)
                => Sum(p.0.into_iter().map(|e| log!(e)).collect()).simplify(),
            int!(1) => Some(int!(0)),
            int!(10) => Some(int!(1)),
            u => Some(u),
        }
    }

    fn simplify_ln(u: Expression) -> Option<Expression> {
        match u {
            Expression::Power(p)
                => prod!(*p.1, ln!(*p.0)).simplify(),
            Expression::Product(p)
                => Sum(p.0.into_iter().map(|e| ln!(e)).collect()).simplify(),
            int!(1) => Some(int!(0)),
            var!("e") => Some(int!(1)),
            u => Some(u),
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