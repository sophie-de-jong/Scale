#![allow(unused_imports)]
#![allow(unused_macros)]

#[macro_use]
mod types;
mod expression;
mod traits;

use expression::Expression;
use types::*;

fn main() {
    let e1 = prod!(
        int!(2),
        var!("a"),
        var!("c"),
        var!("e"),
    );

    let e2 = prod!(
        int!(3),
        var!("b"),
        var!("d"),
        var!("e")
    );

    let expr = prod!(e1, e2);

    println!("Before simplification: {}", expr);

    let simp = expr.simplify().unwrap_or_else(|| {
        println!("Undefined expression");
        std::process::exit(1);
    });
    
    println!("After simplification: {}", simp);

    // let mut v = vec![var!("a"), inv!(var!("c")), var!("d"), var!("f")];
    // v.sort();
    // println!("{:?}", v);
}
