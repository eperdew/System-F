use ::systemf::Expr;
use ::tests::*;

#[test]
fn test1_eval() {
    let expr: Expr = load_and_parse_expr("tests/test1.f");
    let sol:  Expr = load_and_parse_expr("tests/test1_sol.f");

    let result = expr.eval().unwrap();

    println!("Test1:\n\nExpected: {}\nGot: {}\n\n", sol, result.clone());

    assert_eq!(result, sol);
}

#[test]
fn test2_eval() {
    let expr: Expr = load_and_parse_expr("tests/test2.f");
    let sol:  Expr = load_and_parse_expr("tests/test2_sol.f");

    let result = expr.eval();

    assert_eq!(result.unwrap(), sol);
}

#[test]
fn test3_eval() {
    let expr: Expr = load_and_parse_expr("tests/test3.f");
    let sol:  Expr = load_and_parse_expr("tests/test3_sol.f");

    let result = expr.eval();

    assert_eq!(result.unwrap(), sol);
}