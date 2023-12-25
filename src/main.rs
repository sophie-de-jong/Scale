#![allow(unused_imports)]
#![allow(unused_macros)]

#[macro_use]
mod types;
mod expression;
mod traits;

use expression::Expression;
use types::*;

fn main() {
    let expr = pow!(int!(-54), frac!(-2, 3));

    println!("Before simplification: {}", expr);

    let simp = expr.simplify().unwrap_or_else(|| {
        println!("Undefined expression");
        std::process::exit(1);
    });
    
    println!("After simplification: {}", simp);
}
