use crate::expression::{Expression, UndefinedError};
use crate::traits::Simplify;
use crate::types::{self, Power, Integer, Rational, Sum, sum};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Product(Vec<Expression>);

impl Simplify for Product {
    fn simplify(mut self) -> Result<Expression, UndefinedError> {
        self.0 = self.0
            .into_iter()
            .map(Expression::simplify)
            .collect::<Result<Vec<_>, _>>()?;

        self.0.sort();

        if self.0.contains(&int!(0)) {
            return Ok(int!(0))
        }
        
        match self.0.len() {
            0 => Ok(int!(1)),
            1 => Ok(self.take_last().unwrap()), 
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
    pub fn new(values: Vec<Expression>) -> Product {
        Product(values)
    }

    pub fn values(&self) -> &[Expression] {
        self.0.as_slice()
    }

    pub fn term(&self) -> &[Expression] {
        match self.0.first().expect("called `Expression::term()` on an unsimplifed expression") {
            Expression::Integer(_) | Expression::Rational(_) => self.0[1..].as_ref(),
            _ => self.0.as_slice(),
        }
    }

    pub fn coeff(&self) -> Option<&Expression> {
        match self.0.first()? {
            n @ Expression::Integer(_) => Some(n),
            q @ Expression::Rational(_) => Some(q),
            _ => None,
        }
    }

    fn take_last(&mut self) -> Option<Expression> {
        self.0.pop()
    }

    pub fn adjoin(mut self, value: Expression) -> Self {
        self.0.push(value);
        self
    }

    pub fn with_two_args(u1: Expression, u2: Expression) -> Result<Expression, UndefinedError> {
        match (u1, u2) {
            (Expression::Integer(n), q) | (q, Expression::Integer(n)) if n.num() == 1
                => Ok(q),
            
            (Expression::Integer(n), Expression::Integer(m))
                => int!(n.num() * m.num()).simplify(),

            (Expression::Rational(p), Expression::Rational(q))
                => frac!(p.num() * q.num(), p.den() * q.den()).simplify(),
            
            (Expression::Integer(n), Expression::Rational(p)) | (Expression::Rational(p), Expression::Integer(n))
                => frac!(p.num() * n.num(), p.den()).simplify(),

            (u1, u2) if u1.base() == u2.base() => {
                let p = Power::from(u1);
                let q = Power::from(u2);
                pow!(*p.base, sum!(*p.exp, *q.exp).simplify()?).simplify()
            }

            (Expression::Product(p), Expression::Product(q))
                => Product::merge_products(p, q).map(Expression::from),

            (Expression::Product(p), u) | (u, Expression::Product(p))
                => Product::merge_products(p, u.into()).map(Expression::from),

            (u1, u2) if u2 < u1
                => Ok(prod!(u2, u1)),

            (u1, u2) 
                => Ok(prod!(u1, u2))
        }
    }

    fn with_more_args(u0: Expression, p: Product) -> Result<Expression, UndefinedError> {
        let mut result = Product::merge_products(p, u0.into())?;

        match result.0.len() {
            0 => Ok(int!(1)),
            1 => Ok(result.take_last().unwrap()),
            _ => Ok(result.into())
        }
    }

    fn merge_products(mut p: Product, mut q: Product) -> Result<Product, UndefinedError> {
        let Some(p1) = p.take_last() else { return Ok(q) };
        let Some(q1) = q.take_last() else { return Ok(p.adjoin(p1)) };

        match Product::with_two_args(p1.clone(), q1.clone())? {
            Expression::Integer(n) if n.num() == 1 => Product::merge_products(p, q),
            Expression::Product(u) if (u.0.first().unwrap(), u.0.last().unwrap()) == (&p1, &q1) 
                => Ok(Product::merge_products(p.adjoin(p1), q)?.adjoin(q1)),

            Expression::Product(_)
                => Ok(Product::merge_products(p, q.adjoin(q1))?.adjoin(p1)),

            u => Ok(Product::merge_products(p, q)?.adjoin(u)),
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