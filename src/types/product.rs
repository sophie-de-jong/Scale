use std::cmp;
use std::rc::Rc;
use std::slice;

use crate::expression::Expression;
use crate::traits::Simplify;
use crate::types::{self, Power, Integer, Rational, Sum};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Product(pub Rc<[Expression]>);

impl Simplify for Product {
    fn simplify(self) -> Option<Expression> {
        let p = self.0
            .iter()
            .map(|e| e.clone().simplify())
            .collect::<Option<Rc<[_]>>>()?;

        if p.contains(&int!(0)) {
            return Some(int!(0))
        }

        let result = match p.len() {
            0 => int!(1),
            1 => p[0].clone(), 
            2 => Product(Product::with_two_args(p[0].clone(), p[1].clone())?).into(),
            _ => {
                let (u1, u2) = p.split_first()?;
                Product(Product::with_more_args(u1.clone(), u2)?).into()
            }
        };

        Some(result)
    }
}

impl Product {
    fn with_two_args(u1: Expression, u2: Expression) -> Option<Rc<[Expression]>> {
        match (u1, u2) {
            (int!(1), q) | (q, int!(1))
                => Some(Rc::from([q])),
            
            (Expression::Integer(n), Expression::Integer(m)) => {
                match int!(n.0 * m.0) {
                    int!(1) => Some(Rc::from([])),
                    p => Some(Rc::from([p]))
                }
            }
            
            (Expression::Rational(p), Expression::Rational(q)) => {
                match frac!(p.0 * q.0, p.1 * q.1).simplify()? {
                    int!(1) => Some(Rc::from([])),
                    p => Some(Rc::from([p]))
                }
            }
            
            (Expression::Integer(n), Expression::Rational(p)) | (Expression::Rational(p), Expression::Integer(n)) => {
                match frac!(p.0 * n.0, p.1).simplify()? {
                    int!(1) => Some(Rc::from([])),
                    p => Some(Rc::from([p]))
                }
            }
            
            (u1, u2) if u1.base() == u2.base() => {
                let (base1, exp1) = u1.into_base_exp_tuple();
                let (_, exp2) = u2.into_base_exp_tuple();
                Some(Rc::from([pow!(base1, sum!(exp1, exp2).simplify()?).simplify()?]))
            }

            (Expression::Product(p), Expression::Product(q))
                => Some(Product::merge_products(p.0.as_ref(), q.0.as_ref())?),

            (Expression::Product(p), q) | (q, Expression::Product(p), )
                => Some(Product::merge_products(p.0.as_ref(), &[q])?),

            (u1, u2) if u2 < u1
                => Some(Rc::from([u2, u1])),

            (u1, u2) 
                => Some(Rc::from([u1, u2]))
        }
    }

    fn with_more_args(u0: Expression, w: &[Expression]) -> Option<Rc<[Expression]>> {
        match u0 {
            Expression::Product(p) => Some(Product::merge_products(
                p.0.as_ref(),
                w
            )?),
            _ => Some(Product::merge_products(
                &[u0],
                w
            )?),
        }
    }

    fn merge_products<'a>(p: &'a [Expression], q: &'a [Expression]) -> Option<Rc<[Expression]>> {
        let Some((p1, rest_p)) = p.split_first() else { return Some(Rc::from(q)) };
        let Some((q1, rest_q)) = q.split_first() else { return Some(Rc::from(p)) };
        let h = Product::with_two_args(p1.clone(), q1.clone())?;

        match h.as_ref() {
            [] => Product::merge_products(p, q),
            [_] => Some(Rc::from([h.as_ref(), Product::merge_products(rest_p, rest_q)?.as_ref()].concat().as_slice())),
            [p0, q0] if p0 == p1 && q0 == q1 
                => Some(Rc::from([slice::from_ref(p1), Product::merge_products(rest_p, q)?.as_ref()].concat().as_slice())),
            [q0, p0] if p0 == p1 && q0 == q1 
                => Some(Rc::from([slice::from_ref(q1), Product::merge_products(p, rest_q)?.as_ref()].concat().as_slice())),
            [..] => panic!()
        }
    }
}