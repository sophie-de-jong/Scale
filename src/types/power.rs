use std::cmp;
use std::ops::Deref;
use std::rc::Rc;

use crate::expression::Expression;
use crate::traits::Simplify;
use crate::types::{self, Integer, Rational, Product};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Power {
    base: Box<Expression>,
    exp: Box<Expression>
}

impl Simplify for Power {
    fn simplify(self) -> Option<Expression> {
        match (self.base.simplify()?, self.exp.simplify()?) {
            (Expression::Integer(n), Expression::Rational(q))
                => Power::with_radical(n, q),

            (Expression::Integer(n), w) 
                => Power::with_integer_base(n, w),

            (v, Expression::Integer(n)) 
                => Power::with_integer_exp(v, n),
            
            (Expression::Rational(r), w)
                => Some(div!(
                    pow!(int!(r.num()), w.clone()).simplify()?,
                    pow!(int!(r.den()), w).simplify()?
                )),
                
            (v, w) 
                => Some(pow!(v, w)),
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

    pub fn base(&self) -> &Expression {
        self.base.as_ref()
    }

    pub fn exp(&self) -> &Expression {
        self.exp.as_ref()
    }

    fn with_integer_base(n: Integer, w: Expression) -> Option<Expression> {
        match w {
            Expression::Integer(m) if m.num().is_positive() && n.num() == 0
                => Some(int!(0)),

            Expression::Rational(..) if n.num() == 0 => Some(int!(0)),

            _ if n.num() == 0 => None,

            _ if n.num().abs() == 1 => Some(n.into()),

            Expression::Integer(m) if m.num().is_negative()
                => Some(frac!(1, n.num().pow(m.num().unsigned_abs()))),

            Expression::Integer(m)
                => Some(int!(n.num().pow(m.num().unsigned_abs()))),

            w => Some(pow!(n.into(), w))
        }
    }

    fn with_integer_exp(v: Expression, n: Integer) -> Option<Expression> {
        match (v, n) {
            (Expression::Rational(q), n) if n.num().is_negative()
                => Some(frac!(
                    q.den().pow(n.num() as u32), 
                    q.num().pow(n.num() as u32)
                )),

            (Expression::Rational(q), n)
                => Some(frac!(
                    q.num().pow(n.num() as u32),
                    q.den().pow(n.num() as u32) 
                )),

            (_, n) if n.num() == 0
                => Some(int!(1)),

            (v, n) if n.num() == 1
                => Some(v),

            (Expression::Power(p), n) => {
                let u = prod!(*p.exp, n.into()).simplify()?;
                match u {
                    Expression::Integer(n) => Power::with_integer_exp(*p.base, n),
                    _ => Some(pow!(*p.base, u))
                }
            },

            (Expression::Product(r), n)
                => Product::new(r.values()
                    .iter()
                    .map(|v| Power::with_integer_exp(v.clone(), n))
                    .collect::<Option<Vec<_>>>()?
                ).simplify(),
            
            (v, n)
                => Some(pow!(v, n.into()))
        }
    }

    fn with_radical(n: Integer, q: Rational) -> Option<Expression> {
        if q.den() % 2 == 0 && n.num().is_negative() {
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