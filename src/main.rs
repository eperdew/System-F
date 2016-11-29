#![feature(plugin)]

#![plugin(clippy)]

pub mod systemf;
pub mod parser;
pub mod tests;

fn main() {
    let expr = tests::load_and_parse_expr("tests/test1.f");

    let result = expr.eval().unwrap();

    println!("{:?}", result);
}
