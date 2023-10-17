use std::io;

use crate::{parser::Parser, scanner::Scanner};

pub mod parser;
pub mod scanner;
pub mod token;

fn main() {
    // read from stdin
    println!("Enter an expression: (e.g. 1+2*3)");

    let mut expression: String = String::new();

    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line");

    println!("You entered: {}", expression);

    let scanner: Scanner = Scanner::new(expression);
    let mut parser: Parser = Parser::new(scanner);

    let result: bool = parser.parse_exp();

    println!("Expression is {}", if result { "correct" } else { "wrong" });
}
