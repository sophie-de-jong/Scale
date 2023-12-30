#![allow(unused_imports)]
#![allow(unused_macros)]

#[macro_use]
mod types;
mod expression;
mod traits;
mod parser;
mod lexer;
mod tokens;

use lexer::Lexer;
use parser::Parser;

use std::io::{self, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let mut text = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut text).unwrap();

        let tokens = Lexer::new(text).tokens()?;
    
        let expression = Parser::new(tokens).parse()?;

        match expression.simplify() {
            Ok(u) => println!("{}", u),
            Err(e) => println!("{}", e)
        }
    }
}
