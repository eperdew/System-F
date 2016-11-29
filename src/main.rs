#![feature(plugin)]

#![plugin(clippy)]

pub mod systemf;
pub mod parser;

use systemf::Expr;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let test_str = "Let Nat = forall A . (A -> A) -> A -> A in
Let idT = forall A . A -> A in
let id : idT = Lambda A . lambda x : A . x in
let S : Nat -> Nat = 
lambda n : Nat . 
    Lambda A . lambda f : A -> A . lambda x : A . f (n [A] f x) in
let zero : Nat = Lambda A . lambda f : A -> A . lambda x : A . x in
zero [idT] (id [idT -> idT]) (id [idT])";


    let parsed = parser::parse_Expression(test_str).unwrap();

    println!("Parsing {}...\nResult: {:?}", test_str, parser::parse_Expression(test_str).unwrap());
    
    let result = Expr::eval(Rc::new(parsed)).unwrap();

    println!("\n\nEvaluated to: {:?}", result);
}
