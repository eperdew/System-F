#![feature(plugin)]

#![plugin(clippy)]

pub mod systemf;
pub mod parser;
pub mod tests;

use systemf::Expr;
use tests::load_and_parse_expr;

fn main() {
    let expr: Expr = load_and_parse_expr("tests/test4.f");
    
    let result = expr.type_check();

    println!("{:?}\nhas type:\n{:?}",expr, result);
}
