#![feature(plugin)]

#![plugin(clippy)]

pub mod systemf;
pub mod parser;

fn main() {
    let test_str = "lambda x : forall X . X -> X . x";
    println!("Parsing {}...\nResult: {:?}", test_str, parser::parse_Expression(test_str).unwrap());

    println!("Hello, world!");
}
