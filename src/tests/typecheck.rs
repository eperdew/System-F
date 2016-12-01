use ::systemf::*;
use ::tests::*;

#[test]
fn test1_typecheck() {
    let expr: Expr = load_and_parse_expr("tests/test1.f");

    let result = expr.type_check();

    println!("\n{}\nhas type\n{:?}\n", expr, result);
}

#[test]
fn test2_typecheck() {
    let expr: Expr = load_and_parse_expr("tests/test2.f");
    
    let result = expr.type_check();

    println!("\n{}\nhas type\n{}\n", expr, result.unwrap());
}

#[test]
fn test3_typecheck() {
    let expr: Expr = load_and_parse_expr("tests/test3.f");
    
    let result = expr.type_check();

    println!("\n{}\nhas type\n{}\n", expr, result.unwrap());
}

#[test]
fn test4_typecheck() {
    let expr: Expr = load_and_parse_expr("tests/test4.f");
    
    let result = expr.type_check();

    println!("\n{}\nhas type\n{}\n", expr, result.unwrap());
}