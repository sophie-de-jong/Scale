use crate::expression::Expression;

pub trait Simplify {
    fn simplify(self) -> Option<Expression>;
}
