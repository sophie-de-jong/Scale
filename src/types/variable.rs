use std::rc::Rc;

use crate::expression::Expression;
use crate::traits::Simplify;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Variable(&'static str);

impl Simplify for Variable {
    fn simplify(self) -> Option<Expression> {
        Some(self.into())
    }
}

impl Variable {
    pub fn new(name: &'static str) -> Variable {
        Variable(name)
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}
