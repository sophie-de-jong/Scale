use std::iter::zip;
use std::cmp;

use crate::expression::Expression;
use crate::traits::Simplify;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sum(pub Vec<Expression>);

impl Simplify for Sum {
    fn simplify(self) -> Option<Expression> {
        Some(self.simplify_children()?.into())
    }
}

impl Sum {
    fn simplify_children(self) -> Option<Sum> {
        Some(Sum(self.0.into_iter()
            .map(|e| e.simplify())
            .collect::<Option<Vec<_>>>()?
        ))
    }
}