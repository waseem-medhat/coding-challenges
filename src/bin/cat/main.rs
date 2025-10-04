use std::io::{self, IsTerminal, Read};
use std::{env, fs, process};

struct Config {
    content: String,
    print_nums: bool,
}

fn main() {
    let content = if io::stdin().is_terminal() {
        read_from_args()
    } else {
        read_from_stdin()
    };

    let config = match content {
        Err(msg) => {
            println!("{msg}");
            process::exit(1);
        }
        Ok(config) => config,
    };

    if config.print_nums {
        config
            .content
            .lines()
            .enumerate()
            .for_each(|(i, l)| println!(" {:>3} {l}", i + 1))
    } else {
        config.content.lines().for_each(|l| println!("{l}"))
    }
}

fn read_from_args() -> Result<Config, String> {
    let mut args = env::args().skip(1);
    let mut print_nums = false;
    let mut content = String::new();

    // 1st arg should either be -n or a file name
    match args.next() {
        None => return Err(String::from("no file name(s) provided")),
        Some(arg) if arg == String::from("-n") => print_nums = true,
        Some(file_name) => {
            let file_content = fs::read_to_string(file_name).expect("couldn't read file");
            content = file_content;
        }
    }

    args.for_each(|file_name| {
        let file_content = fs::read_to_string(file_name).expect("couldn't read file");
        content += &file_content
    });

    Ok(Config {
        content,
        print_nums,
    })
}

fn read_from_stdin() -> Result<Config, String> {
    let mut content = String::new();
    let result = io::stdin().read_to_string(&mut content);

    if result.is_err() {
        return Err(String::from("couldn't read stdin"));
    }

    let print_nums = env::args().skip(1).next().unwrap_or(String::new()) == "-n";

    Ok(Config {
        content,
        print_nums,
    })
}
