use ::tests::*;

#[test]
fn test1_eval() {
    let expr: Expr = load_and_parse_expr("tests/test1.f");
    let sol:  Expr = load_and_parse_expr("tests/test1_sol.f");

    let result = expr.eval();

    assert_eq!(result, sol);
}

#[test]
fn test2_eval() {
    let expr: Expr = load_and_parse_expr("tests/test2.f");
    let sol:  Expr = load_and_parse_expr("tests/test2_sol.f");

    let result = expr.eval();

    assert_eq!(result, sol);
}

#[test]
fn test3_eval() {
    let expr: Expr = load_and_parse_expr("tests/test3.f");
    let sol:  Expr = load_and_parse_expr("tests/test3_sol.f");

    let result = expr.eval();

    assert_eq!(result, sol);
}

#[test]
fn bool_eval() {
    let expr: Expr = load_and_parse_expr("tests/bool.f");
    let sol:  Expr = load_and_parse_expr("tests/bool_sol.f");

    let result = expr.eval();

    assert_eq!(result, sol);
}