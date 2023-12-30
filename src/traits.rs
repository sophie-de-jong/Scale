use crate::expression::{Expression, UndefinedError};

pub trait Simplify {
    fn simplify(self) -> Result<Expression, UndefinedError>;
}
