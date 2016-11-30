use ::parser;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;
use ::systemf::Expr;

/// Load a file given a file name and parse it to an expression
pub fn load_and_parse_expr(filename: &str) -> Expr {
    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why.description());
    }

    parser::parse_expr(&s).unwrap()
}

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