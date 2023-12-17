use std::fmt;
use std::cmp::Ordering;
use std::rc::Rc;

use crate::traits::Simplify;
use crate::types::{self, Integer};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Expression {
    Integer(types::Integer),
    Rational(types::Rational),
    Variable(types::Variable),
    Sum(types::Sum),
    Product(types::Product),
    Power(types::Power),
}

impl Expression {
    pub fn simplify(self) -> Option<Expression> {
        match self {
            Expression::Integer(i)   => i.simplify(),
            Expression::Rational(r) => r.simplify(),
            Expression::Product(p)   => p.simplify(),
            Expression::Power(p)       => p.simplify(),
            Expression::Sum(s)           => s.simplify(),
            Expression::Variable(v) => v.simplify(),
        }
    }

    pub fn base(&self) -> &Expression {
        match self {
            Expression::Power(p) => p.base(),
            e => e,
        }
    }

    pub fn into_base_exp_tuple(self) -> (Expression, Expression) {
        match self {
            Expression::Power(p) => p.into_tuple(),
            e => (e, int!(1))
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Integer(i) => write!(f, "{}", i.0),
            Expression::Rational(r) => write!(f, "{}/{}", r.0, r.1),
            Expression::Variable(v) => write!(f, "{}", v.0),
            Expression::Power(p) => write!(f, "({}^{})", p.0, p.1),
            Expression::Sum(s) => write!(f, "({})", s.0.iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<_>>()
                .join(" + ")
            ),
            Expression::Product(p) => write!(f, "({})", p.0.iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<_>>()
                .join(" * "))
        }
    }
}

impl From<types::Integer> for Expression {
    fn from(value: types::Integer) -> Self {
        Expression::Integer(value)
    }
}

impl From<types::Rational> for Expression {
    fn from(value: types::Rational) -> Self {
        Expression::Rational(value)
    }
}

impl From<types::Variable> for Expression {
    fn from(value: types::Variable) -> Self {
        Expression::Variable(value)
    }
}

impl From<types::Sum> for Expression {
    fn from(value: types::Sum) -> Self {
        Expression::Sum(value)
    }
}

impl From<types::Product> for Expression {
    fn from(value: types::Product) -> Self {
        Expression::Product(value)
    }
}

impl From<types::Power> for Expression {
    fn from(value: types::Power) -> Self {
        Expression::Power(value)
    }
}