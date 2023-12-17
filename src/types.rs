// The following macros are purposefully defined here before
// any types are imported because macros can only be used after
// they have been defined.

// The purpose of these macros is to allow for easy AST building,
// without needing to mess around with the Expression enum and all
// it's variants. It's less error-prone to simply use a macro.
macro_rules! int {
    ( $x:expr ) => {
        Expression::Integer(Integer($x))
    };
}

macro_rules! frac {
    ( $x:expr, $y:expr ) => {
        Expression::Rational(Rational($x, $y))
    };
}

macro_rules! pow {
    ( $x:expr, $y:expr ) => {
        Expression::Power(Power(Box::new($x), Box::new($y)))
    };
}

macro_rules! sum {
    ($($x:expr),+ $(,)?) => {
        Expression::Sum(Sum(vec![$($x),+]))
    };
}

macro_rules! prod {
    ($($x:expr),+ $(,)?) => {
        Expression::Product(Product(vec![$($x),+]))
    };
}

macro_rules! var {
    ( $x:expr ) => {
        Expression::Variable(Variable($x.to_string()))
    };
}

mod integer;
#[macro_use]
mod rational;
#[macro_use]
mod variable;
#[macro_use]
mod sum;
#[macro_use]
mod product;
#[macro_use]
mod power;

pub use integer::Integer;
pub use rational::Rational;
pub use variable::Variable;
pub use sum::Sum;
pub use product::Product;
pub use power::Power;
