//! # `cat`
//!
//! - Challenge: <https://codingchallenges.fyi/challenges/challenge-cat>
//! - Status: **meets requirements**
//!
//! # TODO
//! - Testing
//! - Feature parity with original tool
//!
use std::fs;
use std::io::{self, IsTerminal, Read};

use clap::Parser;

/// A clone of the `cat` utility
#[derive(Parser, Debug)]
#[command(about, long_about = LONG_ABOUT)]
struct Args {
    filenames: Vec<String>,

    /// Print line numbers
    #[arg(short = 'n')]
    print_nums: bool,

    /// Print non-blank line numbers
    #[arg(short = 'b')]
    print_nums_nonblank: bool,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let content = if io::stdin().is_terminal() {
        read_file_content(&args.filenames)?
    } else {
        read_stdin_content()?
    };

    if args.print_nums_nonblank {
        content
            .lines()
            .fold(1, |line_count, line| match line.is_empty() {
                false => {
                    println!(" {:>3} {line}", line_count);
                    line_count + 1
                }
                true => {
                    println!("     {line}");
                    line_count
                }
            });
        return Ok(());
    }

    if args.print_nums {
        content
            .lines()
            .enumerate()
            .for_each(|(i, l)| println!(" {:>3} {l}", i + 1));
        return Ok(());
    }

    println!("{}", content);
    Ok(())
}

fn read_file_content(filenames: &[String]) -> io::Result<String> {
    let mut content = String::new();
    for filename in filenames {
        content += &fs::read_to_string(filename)?;
    }
    Ok(content)
}

fn read_stdin_content() -> Result<String, io::Error> {
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;
    Ok(content)
}

const LONG_ABOUT: &str = r#"A clone of the `cat` utility

The goal is to achieve feature parity without necessarily perfect 1:1 mapping of the API.

The project was initially guided by Coding Challenges: https://codingchallenges.fyi/challenges/challenge-cat
but then referred directly to `cat` itself.
"#;
