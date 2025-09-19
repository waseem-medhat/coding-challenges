mod config;

use crate::config::Config;
use std::fs;

fn main() {
    let config = Config::from_args();
    let file = fs::read_to_string(config.file_name()).expect("couldn't read file");
    print_field(file, config.field_num());
}

fn print_field(file: String, field_num: u32) {
    file.lines().for_each(|line| {
        let field = line
            .split("\t")
            .nth((field_num - 1).try_into().unwrap())
            .expect("unexpected end of line");
        println!("{field}");
    });
}
