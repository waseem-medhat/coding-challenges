use std::env;
use std::fs;

struct Config {
    filename: String,
    count_opt: CountOpt,
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
        let args: Vec<String> = env::args().collect();
        match args.len() {
            2 => Result::Ok(Self {
                filename: args[1].clone(),
                count_opt: CountOpt::All,
            }),
            3 => {
                let count = match args[1].as_str() {
                    "-c" => CountOpt::Bytes,
                    "-m" => CountOpt::Chars,
                    "-w" => CountOpt::Words,
                    "-l" => CountOpt::Lines,
                    _ => return Err(String::from("invalid arg")),
                };
                Result::Ok(Self {
                    filename: args[2].clone(),
                    count_opt: count,
                })
            }
            _ => Err(String::from("invalid number of args")),
        }
    }
}

fn cmd_count(content: &String, config: &Config) -> String {
    let count_str = match config.count_opt {
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
    };

    format!("{count_str} {}", config.filename)
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

    let file_content = match fs::read_to_string(&config.filename) {
        Err(_) => panic!("couldn't read file!"),
        Ok(str) => str,
    };

    let file_count = cmd_count(&file_content, &config);
    println!("{file_count}");
}
