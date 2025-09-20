use std::env::{self, Args};
use std::fs;
use std::iter::Skip;

pub struct Config {
    field_num: i32, // index starts at 1
    file_name: String,
    delimiter: String,
}

impl Config {
    pub fn from_args() -> Config {
        let args = env::args().skip(1);
        parse(args)
    }

    pub fn field_num(&self) -> i32 {
        self.field_num
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn delimiter(&self) -> String {
        self.delimiter.clone()
    }
}

fn parse(mut args: Skip<Args>) -> Config {
    let mut field_num: Option<i32> = None;
    let mut file_name = None;
    let mut delimiter = None;

    loop {
        match args.next() {
            None => break,
            Some(arg) if arg.starts_with("-f") => {
                let field_num_parsed = arg[2..].parse().unwrap();
                field_num = Some(field_num_parsed);
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
        field_num: field_num.unwrap_or(0),
        file_name: file_name.expect("file name not provided"),
        delimiter: delimiter.unwrap_or(String::from("\t")),
    }
}
