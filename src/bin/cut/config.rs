use std::env::{self, Args};
use std::iter::Skip;

pub struct Config {
    // index starts at 1
    field_num: u32,
}

impl Config {
    pub fn field_num(&self) -> u32 {
        self.field_num
    }

    pub fn from_args() -> Config {
        let mut args = env::args().skip(1);
        parse(&mut args, Config { field_num: 0 })
    }
}

fn parse(args: &mut Skip<Args>, acc: Config) -> Config {
    match args.next() {
        None => acc,
        Some(arg) => {
            if arg.starts_with("-f") {
                let field_num = arg.chars().nth(2).unwrap().to_digit(10).unwrap();
                Config { field_num }
            } else {
                panic!("invalid arg!")
            }
        }
    }
}
