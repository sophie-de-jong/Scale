use std::cmp;

use crate::expression::{Expression, UndefinedError};
use crate::traits::Simplify;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer(i32);

impl Simplify for Integer {
    fn simplify(self) -> Result<Expression, UndefinedError> {
        Ok(self.into())
    }
}

impl Integer {
    pub fn new(n: i32) -> Integer {
        Integer(n)
    }

    pub fn num(&self) -> i32 {
        self.0
    }
}