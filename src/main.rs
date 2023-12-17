#![allow(unused_imports)]
#![allow(unused_macros)]

#[macro_use]
mod types;
mod expression;
mod traits;

use expression::Expression;
use types::*;

fn main() {
    let expr = prod!(
        prod!(
            var!("a"),
            var!("c"),
            var!("e"),
        ),
        prod!(
            var!("a"),
            inv!(var!("c")),
            var!("d"),
            var!("f")
        )
    );

    println!("Before simplification: {}", expr);

    let simp = expr.simplify().unwrap_or_else(|| {
        println!("Undefined expression");
        std::process::exit(1);
    });
    
    println!("After simplification: {}", simp);
}