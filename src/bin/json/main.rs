//! Coding Challenges - JSON Parser
//!
//! - Challenge: <https://codingchallenges.fyi/challenges/challenge-json-parser/>
//! - Status: **meets requirements**
//! - TODO
//!   - Testing (against [full suite](https://www.json.org/JSON_checker/))
//!   - CLI improvements
//!
mod lexer;
mod parser;
mod printer;

use parser::is_valid_json;
use std::env;
use std::fs;
use std::process;

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("filename not provided");
        process::exit(1);
    });

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
