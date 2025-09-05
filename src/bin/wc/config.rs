use std::env;
use std::io::{self, IsTerminal};

pub struct Config {
    input_kind: InputKind,
    count_opt: CountOpt,
}

pub enum InputKind {
    File(String),
    Stdin,
}

pub enum CountOpt {
    Bytes,
    Chars,
    Words,
    Lines,
    All,
}

impl Config {
    pub fn from_args() -> Result<Self, String> {
        let has_stdin = !io::stdin().is_terminal();
        let args: Vec<String> = env::args().collect();

        let (input_kind, count_opt) = match args.len() {
            // input | wc
            1 if has_stdin => (InputKind::Stdin, CountOpt::All),

            // wc <filename>
            2 if !has_stdin => (InputKind::File(args[1].clone()), CountOpt::All),

            // input | wc <flag>
            2 => match arg_to_opt(&args[1]) {
                Ok(opt) => (InputKind::Stdin, opt),
                Err(msg) => return Err(msg),
            },

            //  wc <flag> <filename>
            3 => match arg_to_opt(&args[1]) {
                Ok(opt) => (InputKind::File(args[2].clone()), opt),
                Err(msg) => return Err(msg),
            },

            _ => return Err(String::from("invalid number of args")),
        };

        Ok(Config {
            input_kind,
            count_opt,
        })
    }

    pub fn input_kind(&self) -> &InputKind {
        &self.input_kind
    }

    pub fn count_opt(&self) -> &CountOpt {
        &self.count_opt
    }
}

fn arg_to_opt(arg: &String) -> Result<CountOpt, String> {
    match arg.as_str() {
        "-c" => Ok(CountOpt::Bytes),
        "-m" => Ok(CountOpt::Chars),
        "-w" => Ok(CountOpt::Words),
        "-l" => Ok(CountOpt::Lines),
        _ => return Err(String::from("invalid arg")),
    }
}
