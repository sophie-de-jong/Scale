// The following macros are purposefully defined here before
// any types are imported because macros can only be used after
// they have been defined.

// The purpose of these macros is to allow for easy AST building,
// without needing to mess around with the Expression enum and all
// it's variants. It's less error-prone to simply use a macro.
macro_rules! int {
    ( $x:expr ) => {
        Expression::Integer(Integer::new($x))
    };
}

macro_rules! frac {
    ( $x:expr, $y:expr ) => {
        Expression::Rational(Rational::new($x, $y))
    };
}

macro_rules! pow {
    ( $x:expr, $y:expr ) => {
        Expression::Power(Power::new($x, $y))
    };
}

macro_rules! inv {
    ( $x:expr ) => {
        Expression::Power(Power::new($x, Expression::Integer(Integer::new(-1))))
    };
}

macro_rules! sum {
    ( $($x:expr),+ $(,)? ) => {
        Expression::Sum(Sum::new(vec![$($x),+]))
    };
}

macro_rules! prod {
    ( $($x:expr),+ $(,)? ) => {
        Expression::Product(Product::new(vec![$($x),+]))
    };
}

macro_rules! div {
    ( $x:expr, $y:expr ) => {
        Expression::Product(Product::new(vec![$x, inv!($y)]))
    }
}

macro_rules! neg {
    ( $x:expr ) => {
        Expression::Product(Product::new(vec![Expression::Integer(Integer::new(-1)), $x]))
    };
}

macro_rules! var {
    ( $x:expr ) => {
        Expression::Variable(Variable::new($x))
    };
}

macro_rules! sqrt {
    ( $x:expr ) => {
        Expression::Function(Function::Sqrt(Box::new($x)))
    };
}

macro_rules! cbrt {
    ( $x:expr ) => {
        Expression::Function(Function::Cbrt(Box::new($x)))
    };
}

macro_rules! log {
    ( $x:expr ) => {
        Expression::Function(Function::new("log", $x))
    };
}

macro_rules! ln {
    ( $x:expr ) => {
        Expression::Function(Function::new("ln", $x))
    };
}

macro_rules! func {
    ( $n:expr; $x:expr ) => {
        Expression::Function(Function::new($n, $x))
    };
}

mod integer;
mod rational;
mod variable;
mod sum;
mod product;
mod power;
mod function;

pub use integer::Integer;
pub use rational::Rational;
pub use variable::Variable;
pub use sum::Sum;
pub use product::Product;
pub use power::Power;
pub use function::Function;