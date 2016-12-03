//#![feature(plugin)]

// #![plugin(clippy)]

extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, StoreOption};

pub mod systemf;
pub mod parser;
pub mod tests;

use tests::load_and_parse_expr;

fn main() {
    let mut filename: Option<String> = None;
    let mut eval = false;
    let mut typecheck = false;
    {  
        let mut ap: ArgumentParser = ArgumentParser::new();
        ap.set_description("Interpret or typecheck a System F program");
        ap.refer(&mut eval)
            .add_option(&["-e", "--eval"], StoreTrue,
            "Evaluate the given file");
        ap.refer(&mut typecheck)
            .add_option(&["-t", "--typecheck"], StoreTrue,
            "Typecheck the given file");
        ap.refer(&mut filename)
            .add_argument("filename", StoreOption,
            "The file to evaluate");
        ap.parse_args_or_exit();
    }

    if let Some(name) = filename {
        let e = load_and_parse_expr(&name);

        println!("\nParsed expression:\n\t{}", &e);

        let typ = e.type_check();

        if typecheck || eval {
            match typ {
                Ok(t) => {
                    if typecheck {
                        println!("\nType:\n\t{}", t);
                    }
                },
                Err(e) => {
                    println!("Error typechecking file, {:?}", e);
                    return;
                },
            }
        }

        if eval {
            let res = e.eval();
            println!("\nResult:\n\t{}\n", res);
        }
    }
}
