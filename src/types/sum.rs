use crate::expression::Expression;
use crate::traits::Simplify;
use crate::types::{self, Integer, Rational, Product};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Sum(Vec<Expression>);

impl Simplify for Sum {
    fn simplify(mut self) -> Option<Expression> {
        self.0 = self.0
            .into_iter()
            .map(Expression::simplify)
            .collect::<Option<Vec<_>>>()?;

        self.0.sort();
        
        match self.0.len() {
            0 => Some(int!(1)),
            1 => self.take_last(), 
            2 => Sum::with_two_args(
                self.take_last().unwrap(),
                self.take_last().unwrap()
            ),
            _ => Sum::with_more_args(
                self.take_last().unwrap(),
                self.simplify()?.into()
            )
        }
    }
}

impl Sum {
    pub fn new(values: Vec<Expression>) -> Sum {
        Sum(values)
    }

    pub fn values(&self) -> &[Expression] {
        self.0.as_slice()
    }

    fn take_last(&mut self) -> Option<Expression> {
        self.0.pop()
    }

    fn adjoin(mut self, value: Expression) -> Self {
        self.0.push(value);
        self
    }

    fn with_two_args(u1: Expression, u2: Expression) -> Option<Expression> {
        match (u1, u2) {
            (Expression::Integer(n), q) | (q, Expression::Integer(n)) if n.num() == 0
                => Some(q),
            
            (Expression::Integer(n), Expression::Integer(m))
                => int!(n.num() + m.num()).simplify(),

            (Expression::Rational(p), Expression::Rational(q))
                => frac!(p.num()*q.den() + q.num()*p.den(), p.den() * q.den()).simplify(),
            
            (Expression::Integer(n), Expression::Rational(p)) | (Expression::Rational(p), Expression::Integer(n))
                => frac!(n.num()*p.den() + p.num(), p.den()).simplify(),

            (u1, u2) if u1.term() == u2.term() => {
                let p = Product::from(u1);
                let q = Product::from(u2);
                let s = sum!(
                    p.coeff().unwrap_or(&int!(1)).clone(),
                    q.coeff().unwrap_or(&int!(1)).clone()
                ).simplify()?;
                Product::new(p.term().to_vec()).adjoin(s).simplify()
            }

            (Expression::Sum(p), Expression::Sum(q))
                => Sum::merge_sums(p, q).map(Expression::from),

            (Expression::Sum(p), u) | (u, Expression::Sum(p))
                => Sum::merge_sums(p, u.into()).map(Expression::from),

            (u1, u2) if u2 < u1
                => Some(sum!(u2, u1)),

            (u1, u2) 
                => Some(sum!(u1, u2))
        }
    }

    fn with_more_args(u0: Expression, p: Sum) -> Option<Expression> {
        let mut result = Sum::merge_sums(p, u0.into())?;

        match result.0.len() {
            0 => Some(int!(0)),
            1 => result.take_last(),
            _ => Some(result.into())
        }
    }

    fn merge_sums(mut p: Sum, mut q: Sum) -> Option<Sum> {
        let Some(p1) = p.take_last() else { return Some(q) };
        let Some(q1) = q.take_last() else { return Some(p.adjoin(p1)) };

        match Sum::with_two_args(p1.clone(), q1.clone())? {
            Expression::Integer(n) if n.num() == 0 => Sum::merge_sums(p, q),

            Expression::Sum(u) if (u.0.first()?, u.0.last()?) == (&p1, &q1) 
                => Some(Sum::merge_sums(p.adjoin(p1), q)?.adjoin(q1)),

            Expression::Sum(_)
                => Some(Sum::merge_sums(p, q.adjoin(q1))?.adjoin(p1)),

            u => Some(Sum::merge_sums(p, q)?.adjoin(u)),
        }
    }
}

impl From<Expression> for Sum {
    fn from(value: Expression) -> Self {
        match value {
            Expression::Sum(s) => s,
            u => Sum(vec![u]),
        }
    }
}