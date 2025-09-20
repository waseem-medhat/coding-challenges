use std::env::{self, Args};
use std::fs;
use std::iter::Skip;

pub struct Config {
    field_num: u32, // index starts at 1
    file_name: String,
    delimiter: String,
}

impl Config {
    pub fn from_args() -> Config {
        let mut args = env::args().skip(1);
        parse(
            &mut args,
            Config {
                field_num: 0,
                file_name: String::new(),
                delimiter: String::from("\t"),
            },
        )
    }

    pub fn field_num(&self) -> u32 {
        self.field_num
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn delimiter(&self) -> String {
        self.delimiter.clone()
    }
}

fn parse(args: &mut Skip<Args>, acc: Config) -> Config {
    match args.next() {
        None => acc,

        Some(arg) if arg.starts_with("-f") => {
            let field_num: u32 = arg[2..].parse().expect("invalid num");
            parse(
                args,
                Config {
                    field_num,
                    file_name: acc.file_name(),
                    delimiter: acc.delimiter(),
                },
            )
        }

        Some(arg) if arg.starts_with("-d") => {
            let delimiter = arg[2..].to_string();
            parse(
                args,
                Config {
                    field_num: acc.field_num(),
                    file_name: acc.file_name(),
                    delimiter,
                },
            )
        }

        Some(arg) => match fs::metadata(&arg) {
            Ok(_) => parse(
                args,
                Config {
                    field_num: acc.field_num(),
                    file_name: arg,
                    delimiter: acc.delimiter(),
                },
            ),
            Err(_) => panic!("file {arg} doesn't exist"),
        },
    }
}
