use std::env::{self, Args};
use std::fs;
use std::iter::Skip;

pub struct Config {
    field_nums: Vec<i32>, // index starts at 1
    file_name: String,
    delimiter: String,
}

impl Config {
    pub fn from_args() -> Config {
        let args = env::args().skip(1);
        parse(args)
    }

    pub fn field_nums(&self) -> Vec<i32> {
        self.field_nums.clone()
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn delimiter(&self) -> String {
        self.delimiter.clone()
    }
}

fn parse(mut args: Skip<Args>) -> Config {
    let mut field_nums: Option<Vec<i32>> = None;
    let mut file_name = None;
    let mut delimiter = None;

    loop {
        match args.next() {
            None => break,
            Some(arg) if arg.starts_with("-f") => {
                let field_arg = &arg[2..];
                let nums: Vec<&str> = if field_arg.contains(",") {
                    field_arg.split(",").collect()
                } else if field_arg.contains(" ") {
                    field_arg.split(" ").collect()
                } else {
                    vec![field_arg]
                };

                let parsed_nums: Vec<i32> = nums.iter().map(|n| n.parse().unwrap()).collect();
                field_nums = Some(parsed_nums);
            }
            Some(arg) if arg.starts_with("-d") => {
                let delimiter_parsed = arg[2..].to_string();
                delimiter = Some(delimiter_parsed);
            }
            Some(arg) => match fs::metadata(&arg) {
                Ok(_) => {
                    file_name = Some(arg);
                }
                Err(_) => panic!("file {arg} doesn't exist"),
            },
        }
    }

    Config {
        field_nums: field_nums.unwrap_or(vec![]),
        file_name: file_name.expect("file name not provided"),
        delimiter: delimiter.unwrap_or(String::from("\t")),
    }
}
