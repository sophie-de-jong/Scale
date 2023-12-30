use std::rc::Rc;

use crate::expression::{Expression, UndefinedError};
use crate::traits::Simplify;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Variable(Box<str>);

impl Simplify for Variable {
    fn simplify(self) -> Result<Expression, UndefinedError> {
        Ok(self.into())
    }
}

impl Variable {
    pub fn new(name: impl Into<String>) -> Variable {
        Variable(Box::from(name.into()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}
