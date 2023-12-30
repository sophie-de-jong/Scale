use core::fmt;
use std::cmp;
use std::ops::Deref;
use std::rc::Rc;

use crate::expression::{Expression, UndefinedError};
use crate::traits::Simplify;
use crate::types::{self, Integer, Rational, Product};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Power {
    pub base: Box<Expression>,
    pub exp: Box<Expression>
}

impl Simplify for Power {
    fn simplify(self) -> Result<Expression, UndefinedError> {
        match (self.base.simplify()?, self.exp.simplify()?) {
            (Expression::Integer(n), Expression::Rational(q))
                => Power::with_radical(n, q),

            (Expression::Integer(n), w) 
                => Power::with_integer_base(n, w),

            (v, Expression::Integer(n)) 
                => Power::with_integer_exp(v, n),
            
            (Expression::Rational(r), w)
                => Ok(div!(
                    pow!(int!(r.num()), w.clone()).simplify()?,
                    pow!(int!(r.den()), w).simplify()?
                )),
                
            (v, w) 
                => Ok(pow!(v, w)),
        }
    }
}

impl cmp::Ord for Power {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.base != other.base {
            self.base.cmp(&other.base)
        }
        else {
            self.exp.cmp(&other.exp)
        }
    }
}

impl cmp::PartialOrd for Power {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Power {
    pub fn new(base: Expression, exp: Expression) -> Power {
        Power { base: Box::new(base), exp: Box::new(exp) }
    }

    fn with_integer_base(n: Integer, w: Expression) -> Result<Expression, UndefinedError> {
        match w {
            Expression::Integer(m) if m.num().is_positive() && n.num() == 0
                => Ok(int!(0)),

            Expression::Rational(..) if n.num() == 0 => Ok(int!(0)),

            _ if n.num() == 0 => Err(UndefinedError("Indeterminate form: 0^0".to_string())),

            _ if n.num().abs() == 1 => Ok(n.into()),

            Expression::Integer(m) if m.num().is_negative()
                => Ok(frac!(1, n.num().pow(m.num().unsigned_abs()))),

            Expression::Integer(m)
                => Ok(int!(n.num().pow(m.num().unsigned_abs()))),

            w => Ok(pow!(n.into(), w))
        }
    }

    fn with_integer_exp(v: Expression, n: Integer) -> Result<Expression, UndefinedError> {
        match (v, n) {
            (Expression::Rational(q), n) if n.num().is_negative()
                => Ok(frac!(
                    q.den().pow(n.num() as u32), 
                    q.num().pow(n.num() as u32)
                )),

            (Expression::Rational(q), n)
                => Ok(frac!(
                    q.num().pow(n.num() as u32),
                    q.den().pow(n.num() as u32) 
                )),

            (_, n) if n.num() == 0
                => Ok(int!(1)),

            (v, n) if n.num() == 1
                => Ok(v),

            (Expression::Power(p), n) => {
                let u = prod!(*p.exp, n.into()).simplify()?;
                match u {
                    Expression::Integer(n) => Power::with_integer_exp(*p.base, n),
                    _ => Ok(pow!(*p.base, u))
                }
            },

            (Expression::Product(r), n)
                => Product::new(r.values()
                    .iter()
                    .map(|v| Power::with_integer_exp(v.clone(), n))
                    .collect::<Result<Vec<_>, UndefinedError>>()?
                ).simplify(),
            
            (v, n)
                => Ok(pow!(v, n.into()))
        }
    }

    fn with_radical(n: Integer, q: Rational) -> Result<Expression, UndefinedError> {
        if q.den() % 2 == 0 && n.num().is_negative() {
            return Err(UndefinedError("Negative value under even root".to_string()))
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
            pow!(
                int!(outside_root),
                int!(q.num())
            ).simplify()?,
            pow!(
                Power::with_integer_base(Integer::new(inside_root), int!(q.num()))?,
                frac!(1, q.den())
            )
        )
    }
}

impl From<Expression> for Power {
    fn from(value: Expression) -> Self {
        match value {
            Expression::Power(p) => p,
            u => Power::new(u, int!(1))
        }
    }
}