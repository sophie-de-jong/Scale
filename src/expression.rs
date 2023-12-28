use std::fmt;
use std::cmp;
use std::fmt::Debug;
use std::rc::Rc;

use crate::traits::Simplify;
use crate::types::{self, Integer, Product, Power, Sum};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Integer(types::Integer),
    Rational(types::Rational),
    Variable(types::Variable),
    Sum(types::Sum),
    Product(types::Product),
    Power(types::Power),
    Function(types::Function)
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
            Expression::Function(f) => f.simplify(),
        }
    }

    pub fn base(&self) -> &Expression {
        match self {
            Expression::Power(p) => p.base(),
            e => e,
        }
    }

    pub fn term(&self) -> &[Expression] {
        match self {
            Expression::Product(p) => p.term(),
            u => std::slice::from_ref(u),
        }
    }
}

impl cmp::Ord for Expression {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        use Expression as E;
        match (self, other) {
            (E::Integer(n), E::Integer(m)) => n.cmp(m),
            (E::Integer(_), _) => cmp::Ordering::Less,
            (_, E::Integer(_)) => cmp::Ordering::Greater,
            (E::Rational(r), E::Rational(s)) => r.cmp(s),
            (E::Rational(_), _) => cmp::Ordering::Less,
            (_, E::Rational(_)) => cmp::Ordering::Greater,
            (E::Product(p), E::Product(q)) => p.cmp(q),
            (E::Product(p), u) => p.cmp(&u.clone().into()),
            (u, E::Product(p)) => Product::from(u.clone()).cmp(p),
            (E::Power(p), E::Power(q)) => p.cmp(q),
            (E::Power(p), u) => p.cmp(&u.clone().into()),
            (u, E::Power(p)) => Power::from(u.clone()).cmp(p),
            (E::Sum(s), E::Sum(t)) => s.cmp(t),
            (E::Sum(s), u) => s.cmp(&u.clone().into()),
            (u, E::Sum(s)) => Sum::from(u.clone()).cmp(s),
            (E::Function(f), E::Function(g)) => f.cmp(g),
            (E::Function(f), E::Variable(v)) => f.to_string().cmp(&v.as_str().into()),
            (E::Variable(v), E::Function(f)) => v.as_str().to_owned().cmp(&f.to_string()),
            (E::Variable(v), E::Variable(w)) => v.cmp(w)
        }
    }
} 

impl cmp::PartialOrd for Expression {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Integer(i) => write!(f, "{}", i.num()),
            Expression::Rational(r) => write!(f, "{}/{}", r.num(), r.den()),
            Expression::Power(p) => write!(f, "({})^({})", p.base(), p.exp()),
            Expression::Variable(v) => write!(f, "{}", v.as_str()),
            Expression::Sum(s) => write!(f, "({})", s.values().iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<_>>()
                .join(" + ")
            ),
            Expression::Product(p) => write!(f, "({})", p.values().iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<_>>()
                .join(" * ")),
            Expression::Function(g) => write!(f, "{}", g)
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