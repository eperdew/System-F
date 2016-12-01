use ::systemf::Expr;

#[test]
fn test1_eval() {
    let expr: Expr = load_and_parse_expr("tests/test1.f");
    let sol:  Expr = load_and_parse_expr("tests/test1_sol.f");

    let result = expr.eval();

    assert_eq!(*result.unwrap(), sol);
}

#[test]
fn test2_eval() {
    let expr: Expr = load_and_parse_expr("tests/test2.f");
    let sol:  Expr = load_and_parse_expr("tests/test2_sol.f");

    let result = expr.eval();

    assert_eq!(*result.unwrap(), sol);
}

#[test]
fn test3_eval() {
    let expr: Expr = load_and_parse_expr("tests/test3.f");
    let sol:  Expr = load_and_parse_expr("tests/test3_sol.f");

    let result = expr.eval();

    assert_eq!(*result.unwrap(), sol);
}

#[test]
fn test1_typecheck() {
    let expr: Expr = load_and_parse_expr("tests/test1.f");

    let result = expr.type_check();

    assert!(result.is_some());

    if let Some(res) = result {
        println!("{}\nhas type:\n{}",expr, res);
    }
}

#[test]
fn test2_typecheck() {
    let expr: Expr = load_and_parse_expr("tests/test2.f");
    
    let result = expr.type_check();

    assert!(result.is_some());
    
    if let Some(res) = result {
        println!("{}\nhas type:\n{}",expr, res);
    }
}

#[test]
fn test3_typecheck() {
    let expr: Expr = load_and_parse_expr("tests/test3.f");
    
    let result = expr.type_check();

    assert!(result.is_some());

    if let Some(res) = result {
        println!("{}\nhas type:\n{}",expr, res);
    }
}

#[test]
fn test4_typecheck() {
    let expr: Expr = load_and_parse_expr("tests/test4.f");
    
    let result = expr.type_check();

    assert!(result.is_some());

    if let Some(res) = result {
        println!("{}\nhas type:\n{}",expr, res);
    }
}