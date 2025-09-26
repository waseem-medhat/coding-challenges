use std::env::{self, Args};
use std::io::{IsTerminal, Read};
use std::iter::Skip;
use std::{fs, io};

pub struct Config {
    input: String,
    field_nums: Vec<i32>, // index starts at 1
    delimiter: String,
}

impl Config {
    pub fn from_args() -> Config {
        let args = env::args().skip(1);
        parse(args)
    }

    pub fn input(&self) -> String {
        self.input.clone()
    }

    pub fn field_nums(&self) -> Vec<i32> {
        self.field_nums.clone()
    }

    pub fn delimiter(&self) -> String {
        self.delimiter.clone()
    }
}

fn parse(mut args: Skip<Args>) -> Config {
    let mut field_nums: Option<Vec<i32>> = None;
    let mut delimiter = None;
    let mut input: Option<String> = None;

    loop {
        match args.next() {
            None => break,
            Some(arg) if arg.starts_with("-f") => {
                let fields_arg = &arg[2..];
                field_nums = Some(parse_fields(fields_arg))
            }
            Some(arg) if arg.starts_with("-d") => {
                let delimiter_parsed = arg[2..].to_string();
                delimiter = Some(delimiter_parsed);
            }
            Some(arg) => input = Some(read_file_or_stdin(&arg)),
        }
    }

    Config {
        input: input.expect("no input provided"),
        field_nums: field_nums.unwrap_or(vec![]),
        delimiter: delimiter.unwrap_or(String::from("\t")),
    }
}

fn parse_fields(fields_str: &str) -> Vec<i32> {
    let nums: Vec<&str> = if fields_str.contains(",") {
        fields_str.split(",").collect()
    } else if fields_str.contains(" ") {
        fields_str.split(" ").collect()
    } else {
        vec![fields_str]
    };

    nums.iter().map(|n| n.parse().unwrap()).collect()
}

fn read_file_or_stdin(arg: &String) -> String {
    let mut input = String::new();
    let has_stdin = !io::stdin().is_terminal();

    if arg == &String::from("-") && has_stdin {
        io::stdin().read_to_string(&mut input).unwrap();
        return input;
    }

    match fs::metadata(&arg) {
        Ok(_) => {
            input = fs::read_to_string(arg).unwrap();
            return input;
        }
        Err(_) => panic!("file doesn't exist {arg}"),
    }
}
