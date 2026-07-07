use std::fs;
use std::io::{self, IsTerminal, Read};

use anyhow::Context;

use crate::Args;

pub fn run(args: Args) -> anyhow::Result<()> {
    let is_terminal = io::stdin().is_terminal();
    let content = if is_terminal {
        read_file_content(&args.filename)?
    } else {
        read_stdin_content()?
    };

    let count = count(
        &content,
        args.count_lines,
        args.count_words,
        args.count_bytes,
        args.count_chars,
    );
    if is_terminal {
        println!("{count} {}", args.filename);
    } else {
        println!("{count}");
    }

    Ok(())
}

fn read_file_content(filename: &str) -> anyhow::Result<String> {
    fs::read_to_string(filename).with_context(|| format!("Couldn't read {}", filename))
}

fn read_stdin_content() -> anyhow::Result<String> {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .with_context(|| "Couldn't read from stdin")?;
    Ok(content)
}

fn count(content: &str, lines: bool, words: bool, bytes: bool, chars: bool) -> String {
    if lines {
        count_lines(content).to_string()
    } else if words {
        count_words(content).to_string()
    } else if chars {
        count_chars(content).to_string()
    } else if bytes {
        count_bytes(content).to_string()
    } else {
        let bytes = count_bytes(content).to_string();
        let words = count_words(content).to_string();
        let lines = count_lines(content).to_string();

        format!(" {:>7} {:>7} {:>7}", lines, words, bytes)
    }
}

fn count_bytes(content: &str) -> usize {
    content.len()
}

fn count_lines(content: &str) -> usize {
    content.chars().filter(|&c| c == '\n').count()
}

fn count_words(content: &str) -> usize {
    content.split_ascii_whitespace().count()
}

fn count_chars(content: &str) -> usize {
    content.chars().count()
}
