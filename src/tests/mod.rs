// pub mod eval;
// pub mod typecheck;
pub mod types;

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