use std::cmp;

use crate::expression::Expression;
use crate::traits::Simplify;
use crate::types::{self, Power, Integer, Rational, Sum};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Product(pub Vec<Expression>);

impl Simplify for Product {
    fn simplify(self) -> Option<Expression> {
        let mut p = self.0
            .into_iter()
            .map(|e| e.simplify())
            .collect::<Option<Vec<_>>>()?;

        if p.contains(&int!(0)) {
            return Some(int!(0))
        }

        let result = match p.len() {
            0 => int!(1),
            1 => p.pop().unwrap(), 
            2 => Product(Product::with_two_args(
                p.pop().unwrap(),
                p.pop().unwrap()
            )?).into(),
            _ => Product(Product::with_more_args(
                p.pop().unwrap(),
                p
            )?).into()
        };

        Some(result)
    }
}

impl Product {
    fn with_two_args(u1: Expression, u2: Expression) -> Option<Vec<Expression>> {
        match (u1, u2) {
            (Expression::Product(p), Expression::Product(q))
                => Some(Product::merge_products(p.0, q.0)?),

            (Expression::Product(p), q) | (q, Expression::Product(p), )
                => Some(Product::merge_products(p.0, vec![q])?),

            (int!(1), q) | (q, int!(1))
                => Some(vec![q.simplify()?]),
            
            (Expression::Integer(n), Expression::Integer(m))
                => Some(vec![int!(n.0 * m.0)]),

            (Expression::Rational(p), Expression::Rational(q))
                => Some(vec![frac!(p.0 * q.0, p.1 * q.1).simplify()?]),

            (Expression::Integer(n), Expression::Rational(p)) | (Expression::Rational(p), Expression::Integer(n))
                => Some(vec![frac!(p.0 * n.0, p.1).simplify()?]),

            (u1, u2) if u1.base() == u2.base()
                => Some(vec![pow!(u1.base(), sum!(u1.exponent(), u2.exponent()).simplify()?)]),

            (u1, u2) if u2 < u1
                => Some(vec![u2, u1]),

            (u1, u2) 
                => Some(vec![u1, u2])
        }
    }

    fn with_more_args(u0: Expression, w: Vec<Expression>) -> Option<Vec<Expression>> {
        match u0 {
            Expression::Product(p) => Some(Product::merge_products(
                p.0,
                w
            )?),
            _ => Some(Product::merge_products(
                vec![u0],
                w
            )?),
        }
    }

    fn merge_products(mut p: Vec<Expression>, mut q: Vec<Expression>) -> Option<Vec<Expression>> {
        let Some(p1) = p.pop() else { return Some(q) };
        let Some(q1) = q.pop() else { return Some(p) };
        let mut h = Product::with_two_args(p1, q1)?;

        match h.len() {
            0 => Product::merge_products(p, q),
            1 => {
                let mut v = Product::merge_products(p, q)?;
                v.push(h.pop().unwrap());
                Some(v)
            }
            2 => {
                todo!()
            }
            _ => Some(h)
        }
    }
}