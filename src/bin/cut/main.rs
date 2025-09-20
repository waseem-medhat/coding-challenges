mod config;

use crate::config::Config;
use std::fs;

fn main() {
    let config = Config::from_args();
    let file = fs::read_to_string(config.file_name()).expect("couldn't read file");
    print_field(file, config.field_nums(), config.delimiter());
}

fn print_field(file: String, field_nums: Vec<i32>, delimiter: String) {
    file.lines().for_each(|line| {
        let cut_line: Vec<&str> = line
            .split(&delimiter)
            .enumerate()
            .filter(|(i, _)| field_nums.contains(&(i + 1).try_into().unwrap()))
            .map(|(_, s)| s)
            .collect();

        println!("{}", cut_line.join(&delimiter));
    });
}
