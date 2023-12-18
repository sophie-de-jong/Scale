use std::cmp;
use std::rc::Rc;

use crate::expression::Expression;
use crate::traits::Simplify;
use crate::types::{self, Integer, Rational, Product};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Power(pub Box<Expression>, pub Box<Expression>);

impl Simplify for Power {
    fn simplify(self) -> Option<Expression> {
        match (self.0.simplify()?, self.1.simplify()?) {
            (Expression::Integer(n), w) 
                => Power::with_integer_base(n, w),

            (v, Expression::Integer(n)) 
                => Power::with_integer_exp(v, n),

            (v, w) 
                => Some(pow!(v, w)),
        }
    }
}

impl cmp::Ord for Power {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.base() != other.base() {
            self.base().cmp(other.base())
        }
        else {
            self.exponent().cmp(other.exponent())
        }
    }
}


impl cmp::PartialOrd for Power {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Power {
    pub fn base(&self) -> &Expression {
        self.0.as_ref()
    }

    pub fn exponent(&self) -> &Expression {
        self.1.as_ref()
    }

    fn with_integer_base(n: Integer, w: Expression) -> Option<Expression> {
        match (n, w) {
            (Integer(0), Expression::Integer(m)) if m.is_pos()
                => Some(int!(0)),

            (Integer(0), Expression::Rational(..)) 
                => Some(int!(0)),

            (Integer(0), _) 
                => None,

            (Integer(1), _) 
                => Some(int!(1)),

            (n, Expression::Integer(m)) if m.is_neg()
                => Some(frac!(1, n.0.pow(m.0 as u32))),

            (n, Expression::Integer(m))
                => Some(int!(n.0.pow(m.0 as u32))),

            (n, w)
                => Some(pow!(n.into(), w))
        }
    }

    fn with_integer_exp(v: Expression, n: Integer) -> Option<Expression> {
        match (v, n) {
            (Expression::Rational(q), n) if n.is_neg()
                => Some(frac!(
                    q.1.pow(n.0 as u32), 
                    q.0.pow(n.0 as u32)
                )),

            (Expression::Rational(q), n)
                => Some(frac!(
                    q.0.pow(n.0 as u32),
                    q.1.pow(n.0 as u32) 
                )),

            (_, Integer(0))
                => Some(int!(1)),

            (v, Integer(1))
                => Some(v),

            (Expression::Power(Power(r, s)), n) => {
                let p = prod!(*s, n.into()).simplify()?;
                match p {
                    Expression::Integer(n) => Power::with_integer_exp(*r, n),
                    _ => Some(pow!(*r, p))
                }
            },

            (Expression::Product(r), n)
                => Product(r.0
                    .iter()
                    .map(|v| Power::with_integer_exp(v.clone(), n))
                    .collect::<Option<Vec<_>>>()?
                ).simplify(),
            
            (v, n)
                => Some(pow!(v, n.into()))
        }
    }
}