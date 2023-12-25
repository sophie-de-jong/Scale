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
            (Expression::Integer(n), Expression::Rational(q))
                => Power::with_radical(n, q),

            (Expression::Integer(n), w) 
                => Power::with_integer_base(n, w),

            (v, Expression::Integer(n)) 
                => Power::with_integer_exp(v, n),

            (Expression::Rational(r), Expression::Rational(s))
                => Some(prod!(
                    Power::with_radical(Integer(r.num()), s)?,
                    Power::with_radical(Integer(r.den()), s)?
                )),

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

            (Integer(n), _) if n.abs() == 1
                => Some(int!(n)),

            (n, Expression::Integer(m)) if m.is_neg()
                => Some(frac!(1, n.0.pow(m.0.unsigned_abs()))),

            (n, Expression::Integer(m))
                => Some(int!(n.0.pow(m.0.unsigned_abs()))),

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

    fn with_radical(n: Integer, q: Rational) -> Option<Expression> {
        if q.den() % 2 == 0 && n.is_neg() {
            return None
        }

        let mut outside_root = 1;
        let mut inside_root = n.num();
        let exp = q.den().unsigned_abs();
        let mut d = 2i32;
        let mut e = d.pow(exp);

        while e <= inside_root.abs() {
            if inside_root % e == 0 {
                inside_root /= e;
                outside_root *= d;
            }
            else {
                d += 1;
                e = d.pow(exp);
            }
        }

        Product::with_two_args(
            pow!(int!(outside_root), int!(q.num())).simplify()?,
            Power::with_integer_base(Integer(inside_root), q.into())?
        )
    }
}

impl From<Expression> for Power {
    fn from(value: Expression) -> Self {
        match value {
            Expression::Power(p) => p,
            u => Power(Box::new(u), Box::new(int!(1)))
        }
    }
}