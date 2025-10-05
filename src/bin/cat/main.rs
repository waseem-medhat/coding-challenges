use std::io::{self, IsTerminal, Read};
use std::{env, fs};

struct Config {
    content: String,
    print_nums: bool,
}

fn main() -> Result<(), io::Error> {
    let config = if io::stdin().is_terminal() {
        read_from_args()?
    } else {
        read_from_stdin()?
    };

    if config.print_nums {
        config
            .content
            .lines()
            .enumerate()
            .for_each(|(i, l)| println!(" {:>3} {l}", i + 1))
    } else {
        println!("{}", config.content)
    }

    Ok(())
}

fn read_from_args() -> Result<Config, io::Error> {
    let mut args = env::args().skip(1);
    let mut print_nums = false;
    let mut content = String::new();

    // 1st arg should either be -n or a file name
    match args.next() {
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "no arguments provided",
            ));
        }
        Some(arg) if arg == "-n" => print_nums = true,
        Some(file_name) => {
            content += &fs::read_to_string(file_name)?;
        }
    }

    while let Some(file_name) = args.next() {
        content += &fs::read_to_string(file_name)?
    }

    Ok(Config {
        content,
        print_nums,
    })
}

fn read_from_stdin() -> Result<Config, io::Error> {
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;

    let print_nums = env::args().skip(1).next().unwrap_or(String::new()) == "-n";

    Ok(Config {
        content,
        print_nums,
    })
}
