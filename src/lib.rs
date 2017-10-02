#[macro_use]
extern crate nom;

mod types;
mod parser;
mod evaluator;

use parser::expr;

pub fn main() {
    println!("{:?}", expr("(1 + 2) * 4"));
}
