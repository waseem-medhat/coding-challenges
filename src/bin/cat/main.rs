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
    let args: Vec<String> = env::args().skip(1).collect();
    let (file_names, print_nums) = if args[0] == String::from("-n") {
        (&args[1..], true)
    } else {
        (&args[..], false)
    };

    if let [] = file_names {
        return Err(String::from("no file name(s) provided"));
    }

    let content = file_names.iter().fold(String::new(), |acc, file_name| {
        let file_content = fs::read_to_string(file_name).expect("couldn't read file");
        acc + &file_content
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

    let args: Vec<String> = env::args().skip(1).collect();
    let print_nums = args.len() > 0 && args[0] == String::from("-n");

    Ok(Config {
        content,
        print_nums,
    })
}
