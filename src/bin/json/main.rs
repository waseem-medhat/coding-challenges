mod lexer;
mod parser;
mod printer;

use parser::is_valid_json;
use std::env;
use std::fs;
use std::process;

// use crate::parser::valid_json;

fn main() {
    let filename = match env::args().nth(1) {
        None => {
            eprintln!("filename not provided");
            process::exit(1);
        }
        Some(filename) => filename,
    };

    let input = fs::read_to_string(filename).expect("couldn't read file");
    let tokens = lexer::lex(&input).expect("tokenization error");
    printer::print(&tokens);
    if is_valid_json(&tokens) {
        println!("valid!")
    } else {
        eprintln!("parsing error");
        process::exit(1)
    }
}
