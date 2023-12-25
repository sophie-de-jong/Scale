use crate::expression::Expression;
use crate::traits::Simplify;
use crate::types::{self, Power, Integer, Rational, Sum};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Product(pub Vec<Expression>);

impl Simplify for Product {
    fn simplify(mut self) -> Option<Expression> {
        self.0 = self.0
            .into_iter()
            .map(Expression::simplify)
            .collect::<Option<Vec<_>>>()?;

        self.0.sort();

        if self.0.contains(&int!(0)) {
            return Some(int!(0))
        }
        
        match self.0.len() {
            0 => Some(int!(1)),
            1 => self.take_last(), 
            2 => Product::with_two_args(
                self.take_last().unwrap(),
                self.take_last().unwrap()
            ),
            _ => Product::with_more_args(
                self.take_last().unwrap(),
                self.simplify()?.into()
            )
        }
    }
}

impl Product {
    pub fn term(&self) -> &[Expression] {
        match self.0.first().expect("called `Expression::term()` on an unsimplifed expression") {
            Expression::Integer(_) | Expression::Rational(_) => self.0[1..].as_ref(),
            _ => self.0.as_slice(),
        }
    }

    pub fn coeff(&self) -> &Expression {
        match self.0.first().expect("called `Expression::coeff()` on an unsimplifed expression") {
            n @ Expression::Integer(_) => n,
            q @ Expression::Rational(_) => q,
            _ => &int!(1),
        }
    }

    fn take_last(&mut self) -> Option<Expression> {
        self.0.pop()
    }

    pub fn adjoin(mut self, value: Expression) -> Self {
        self.0.push(value);
        self
    }

    pub fn with_two_args(u1: Expression, u2: Expression) -> Option<Expression> {
        match (u1, u2) {
            (int!(1), q) | (q, int!(1))
                => Some(q),
            
            (Expression::Integer(n), Expression::Integer(m))
                => int!(n.num() * m.num()).simplify(),

            (Expression::Rational(p), Expression::Rational(q))
                => frac!(p.num() * q.num(), p.den() * q.den()).simplify(),
            
            (Expression::Integer(n), Expression::Rational(p)) | (Expression::Rational(p), Expression::Integer(n))
                => frac!(p.num() * n.num(), p.den()).simplify(),

            (u1, u2) if u1.base() == u2.base()
                => pow!(
                    u1.base().clone(), 
                    sum!(u1.exponent().clone(), u2.exponent().clone()).simplify()?
                ).simplify(),

            (Expression::Product(p), Expression::Product(q))
                => Product::merge_products(p, q).map(Expression::from),

            (Expression::Product(p), u) | (u, Expression::Product(p))
                => Product::merge_products(p, u.into()).map(Expression::from),

            (u1, u2) if u2 < u1
                => Some(prod!(u2, u1)),

            (u1, u2) 
                => Some(prod!(u1, u2))
        }
    }

    fn with_more_args(u0: Expression, p: Product) -> Option<Expression> {
        let mut result = Product::merge_products(p, u0.into())?;

        match result.0.len() {
            0 => Some(int!(1)),
            1 => result.take_last(),
            _ => Some(result.into())
        }
    }

    fn merge_products(mut p: Product, mut q: Product) -> Option<Product> {
        let Some(p1) = p.take_last() else { return Some(q) };
        let Some(q1) = q.take_last() else { return Some(p.adjoin(p1)) };

        match Product::with_two_args(p1.clone(), q1.clone())? {
            int!(1) => Product::merge_products(p, q),

            Expression::Product(u) if (u.0.first()?, u.0.last()?) == (&p1, &q1) 
                => Some(Product::merge_products(p.adjoin(p1), q)?.adjoin(q1)),

            Expression::Product(_)
                => Some(Product::merge_products(p, q.adjoin(q1))?.adjoin(p1)),

            u => Some(Product::merge_products(p, q)?.adjoin(u)),
        }
    }
}

impl From<Expression> for Product {
    fn from(value: Expression) -> Self {
        match value {
            Expression::Product(p) => p,
            u => Product(vec![u]),
        }
    }
}