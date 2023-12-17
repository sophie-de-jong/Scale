#![allow(unused_imports)]

#[macro_use]
mod types;
mod expression;
mod traits;

use expression::Expression;
use types::*;

fn main() {
    let expr = prod!(
        frac!(-1, 2),
        int!(10),
        int!(-1),
        pow!(var!("x"), int!(3))
    );

    let simp = expr.simplify().unwrap_or_else(|| {
        println!("Undefined expression");
        std::process::exit(1);
    });
    
    println!("{}", simp);
}