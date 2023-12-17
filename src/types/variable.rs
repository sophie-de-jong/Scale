use std::cmp;

use crate::expression::Expression;
use crate::traits::Simplify;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Variable(pub String);

impl Simplify for Variable {
    fn simplify(self) -> Option<Expression> {
        Some(self.into())
    }
}
