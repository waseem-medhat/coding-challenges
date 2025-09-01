use std::env;
use std::fs;
use std::io;
use std::io::IsTerminal;
use std::io::Read;

struct Config {
    input_kind: InputKind,
    count_opt: CountOpt,
}

enum InputKind {
    File(String),
    Stdin,
}

enum CountOpt {
    Bytes,
    Chars,
    Words,
    Lines,
    All,
}

impl Config {
    fn from_args() -> Result<Self, String> {
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

fn cmd_count(content: &String, config: &Config) -> String {
    match config.count_opt {
        CountOpt::Bytes => count_bytes(content).to_string(),
        CountOpt::Lines => count_lines(content).to_string(),
        CountOpt::Words => count_words(content).to_string(),
        CountOpt::Chars => count_chars(content).to_string(),
        CountOpt::All => {
            let bytes = count_bytes(content).to_string();
            let words = count_words(content).to_string();
            let lines = count_lines(content).to_string();

            format!(" {:>7} {:>7} {:>7}", lines, words, bytes)
        }
    }
}

fn count_bytes(content: &String) -> usize {
    content.len()
}

fn count_lines(content: &String) -> usize {
    content.chars().filter(|&c| c == '\n').count()
}

fn count_words(content: &String) -> usize {
    content.split_ascii_whitespace().count()
}

fn count_chars(content: &String) -> usize {
    content.chars().count()
}

fn main() {
    let config = match Config::from_args() {
        Err(msg) => panic!("{msg}; usage: wc [option] <filename>"),
        Ok(config) => config,
    };

    let content = match config.input_kind {
        InputKind::File(ref filename) => match fs::read_to_string(filename) {
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

    let count = cmd_count(&content, &config);
    match config.input_kind {
        InputKind::File(filename) => println!("{count} {filename}"),
        InputKind::Stdin => println!("{count}"),
    }
}
