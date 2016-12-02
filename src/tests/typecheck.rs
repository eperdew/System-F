use systemf::{Type,Expr};
use ::tests::*;

#[test]
fn test1_typecheck() {
    let expr: Expr = load_and_parse_expr("tests/test1.f");

    let result = expr.type_check();

    let expected = Type::Forall(
                        String::from("A"),
                        Box::new(Type::Fun(
                            Box::new(Type::Var(String::from("A"))),
                            Box::new(Type::Var(String::from("A"))))));

    if let Ok(res) = result {
        assert_eq!(res, expected);
    } else {
        assert!(false);
    }
}

#[test]
fn test2_typecheck() {
    let expr: Expr = load_and_parse_expr("tests/test2.f");
    
    let result = expr.type_check();

    let expected = Type::Fun(
        Box::new(Type::Forall(String::from("X"),
            Box::new(Type::Forall(String::from("Y"),
                Box::new(Type::Forall(String::from("Z"),
                    Box::new(Type::Var(String::from("X"))))))))),
        Box::new(Type::Forall(String::from("X"),
            Box::new(Type::Forall(String::from("Y"),
                Box::new(Type::Forall(String::from("Z"),
                    Box::new(Type::Var(String::from("X"))))))))));
                    
    if let Ok(res) = result {
        assert_eq!(res, expected);
    } else {
        assert!(false);
    }
}

#[test]
fn test3_typecheck() {
    let expr: Expr = load_and_parse_expr("tests/test3.f");
    
    let result = expr.type_check();

    let expected = Type::Fun(
        Box::new(Type::Forall(String::from("Y"),
            Box::new(Type::Forall(String::from("X"),
                Box::new(Type::Var(String::from("X"))))))),
        Box::new(Type::Forall(String::from("Y"),
            Box::new(Type::Forall(String::from("X"),
                Box::new(Type::Var(String::from("X"))))))));

    if let Ok(res) = result {
        assert_eq!(res, expected);
    } else {
        assert!(false);
    }
}

#[test]
fn test4_typecheck() {
    let expr: Expr = load_and_parse_expr("tests/test4.f");
    
    let result = expr.type_check();

    let expected = Type::Forall(String::from("X"),
        Box::new(Type::Fun(
            Box::new(Type::Var(String::from("X"))),
            Box::new(Type::Var(String::from("X"))))));

    if let Ok(res) = result {
        assert_eq!(res, expected);
    } else {
        assert!(false);
    }
}

#[test]
fn almost_omega_typecheck() {
    let expr: Expr = load_and_parse_expr("tests/almost_omega.f");

    let result = expr.type_check();

    let expected = Type::Fun(
        Box::new(Type::Forall(String::from("X"),
        Box::new(Type::Fun(
            Box::new(Type::Var(String::from("X"))),
            Box::new(Type::Var(String::from("X"))))))),
        Box::new(Type::Forall(String::from("X"),
        Box::new(Type::Fun(
            Box::new(Type::Var(String::from("X"))),
            Box::new(Type::Var(String::from("X"))))))));

    if let Ok(res) = result {
        assert_eq!(res, expected);
    } else {
        assert!(false);
    }
}

#[test]
fn bool_typecheck() {
    let expr: Expr = load_and_parse_expr("tests/bool.f");

    let result = expr.type_check();

    let expected = Type::Forall("A".to_string(),
        Box::new(Type::Fun(
            Box::new(Type::Var("A".to_string())),
            Box::new(Type::Fun(
                Box::new(Type::Var("A".to_string())),
                Box::new(Type::Var("A".to_string()))
            ))
        )));

    if let Ok(res) = result {
        assert_eq!(res, expected);
    } else {
        assert!(false);
    }
}