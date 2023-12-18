use std::cmp;
use std::rc::Rc;

use crate::expression::Expression;
use crate::traits::Simplify;
use crate::types::{self, Integer};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Sum(pub Vec<Expression>);

impl Simplify for Sum {
    fn simplify(self) -> Option<Expression> {
        let p = self.0
            .iter()
            .map(|e| e.clone().simplify())
            .collect::<Option<Vec<_>>>()?;

        let mut sum = 0;
        for i in p.iter() {
            match i {
                Expression::Integer(n) => sum += n.0,
                _ => return Some(Sum(p).into()),
            }
        }

        Some(int!(sum))
    }
}
