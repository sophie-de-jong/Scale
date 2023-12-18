use std::cmp;

use crate::expression::Expression;
use crate::traits::Simplify;
use crate::types::{self, Power, Integer, Rational, Sum};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Product(pub Vec<Expression>);

impl Simplify for Product {
    fn simplify(mut self) -> Option<Expression> {
        self.0
            .iter()
            .map(|e| e.clone().simplify())
            .collect::<Option<Vec<_>>>()?;

        if self.0.contains(&int!(0)) {
            return Some(int!(0))
        }
        
        match self.0.len() {
            0 => Some(int!(1)),
            1 => self.take_last(), 
            2 => Product::with_two_args(self.take_last().unwrap(), self.take_last().unwrap()),
            _ => Product::with_more_args(self.take_last().unwrap(), self)
        }
    }
}

impl Product {
    fn take_last(&mut self) -> Option<Expression> {
        self.0.pop()
    }

    fn adjoin(mut self, value: Expression) -> Self {
        self.0.push(value);
        self
    }

    fn with_two_args(u1: Expression, u2: Expression) -> Option<Expression> {
        match (u1, u2) {
            (int!(1), q) | (q, int!(1))
                => Some(q),
            
            (Expression::Integer(n), Expression::Integer(m))
                => Some(int!(n.0 * m.0)),

            (Expression::Rational(p), Expression::Rational(q))
                => Some(frac!(p.0 * q.0, p.1 * q.1).simplify()?),
            
            (Expression::Integer(n), Expression::Rational(p)) | (Expression::Rational(p), Expression::Integer(n))
                => Some(frac!(p.0 * n.0, p.1).simplify()?),

            (u1, u2) if u1.base() == u2.base()
                => Some(pow!(
                    u1.base().clone(), 
                    sum!(u1.exponent().clone(), u2.exponent().clone()).simplify()?
                ).simplify()?),

            (Expression::Product(p), Expression::Product(q))
                => Some(Product::merge_products(p, q)?.into()),

            (Expression::Product(p), q) | (q, Expression::Product(p), )
                => Some(Product::merge_products(p, Product(vec![q]))?.into()),

            (u1, u2) if u2 < u1
                => Some(prod!(u2, u1)),

            (u1, u2) 
                => Some(prod!(u1, u2))
        }
    }

    fn with_more_args(u0: Expression, p: Product) -> Option<Expression> {
        match u0 {
            Expression::Product(q) 
                => Some(Product::merge_products(p, q)?.into()),

            _ 
                => Some(Product::merge_products(p, Product(vec![u0]))?.into()),
        }
    }

    fn merge_products(mut p: Product, mut q: Product) -> Option<Product> {
        let Some(p1) = p.take_last() else { return Some(q) };
        let Some(q1) = q.take_last() else { return Some(p.adjoin(p1)) };

        match Product::with_two_args(p1.clone(), q1.clone())? {
            int!(1) => Product::merge_products(p, q),

            Expression::Product(u) if u.0.as_slice() == [p1.clone(), q1.clone()] 
                => Some(Product::merge_products(p, q.adjoin(p1))?.adjoin(q1)),

            Expression::Product(u) if u.0.as_slice() == [q1.clone(), p1.clone()] 
                => Some(Product::merge_products(p.adjoin(q1), q)?.adjoin(p1)),

            u => Some(Product::merge_products(p, q)?.adjoin(u)),
        }
    }
}