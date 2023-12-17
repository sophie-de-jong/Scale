use std::cmp;
use std::rc::Rc;

use crate::expression::Expression;
use crate::traits::Simplify;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Variable(pub Rc<str>);

impl Simplify for Variable {
    fn simplify(self) -> Option<Expression> {
        Some(self.into())
    }
}
