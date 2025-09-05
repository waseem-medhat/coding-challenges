mod cmd;
mod config;

use config::{Config, InputKind};
use std::fs;
use std::io::{self, Read};

fn main() {
    let config = match Config::from_args() {
        Err(msg) => panic!("{msg}; usage: wc [option] <filename>"),
        Ok(config) => config,
    };

    let content = match config.input_kind() {
        InputKind::File(filename) => match fs::read_to_string(filename) {
            Err(err) => panic!("couldn't read file {filename}: {:?}", err),
            Ok(str) => str,
        },
        InputKind::Stdin => {
            let mut stdin_string = String::new();
            io::stdin()
                .read_to_string(&mut stdin_string)
                .expect("couldn't read stdin");
            stdin_string
        }
    };

    let count = cmd::count(&content, config.count_opt());
    match config.input_kind() {
        InputKind::File(filename) => println!("{count} {filename}"),
        InputKind::Stdin => println!("{count}"),
    }
}
