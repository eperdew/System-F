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