use std::cmp;

use crate::expression::Expression;
use crate::traits::Simplify;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer(pub i64);

impl Simplify for Integer {
    fn simplify(self) -> Option<Expression> {
        Some(self.into())
    }
}

impl Integer {
    pub fn is_pos(&self) -> bool {
        self.0 > 0
    }

    pub fn is_neg(&self) -> bool {
        self.0 < 0
    }
}