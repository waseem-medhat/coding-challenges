use std::env;
use std::fs;

struct Config {
    filename: String,
    count: Count,
}

enum Count {
    Bytes,
    Chars,
    Words,
    Lines,
    All,
}

impl Config {
    fn from_args() -> Self {
        let args: Vec<String> = env::args().collect();
        match args.len() {
            2 => Self {
                filename: args[1].clone(),
                count: Count::All,
            },
            3 => {
                let count = match args[1].as_str() {
                    "-c" => Count::Bytes,
                    "-m" => Count::Chars,
                    "-w" => Count::Words,
                    "-l" => Count::Lines,
                    _ => panic!("invalid arg"),
                };
                Self {
                    filename: args[2].clone(),
                    count,
                }
            }
            _ => panic!("invalid number of args; usage: wc [option] <filename>"),
        }
    }
}

fn count(content: String, count: Count) -> usize {
    match count {
        Count::Bytes => content.len(),
        Count::Lines => content.chars().filter(|&c| c == '\n').count(),
        Count::Words => content.split_ascii_whitespace().count(),
        _ => panic!("TODO :D"),
    }
}

fn main() {
    let config = Config::from_args();
    let file_content = match fs::read_to_string(config.filename) {
        Err(_) => panic!("couldn't read file!"),
        Ok(str) => str,
    };

    let file_count = count(file_content, config.count);
    println!("{file_count}");
}
