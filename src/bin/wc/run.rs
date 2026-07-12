use std::slice;

use crate::Args;
use coding_challenges::helpers::{read_file_content, read_stdin_content};

pub fn run(args: Args) -> anyhow::Result<()> {
    let is_reading_stdin = args.filename.is_empty();

    let content = if is_reading_stdin {
        read_stdin_content()?
    } else {
        read_file_content(slice::from_ref(&args.filename))?
    };

    let count = count(
        &content,
        args.count_lines,
        args.count_words,
        args.count_bytes,
        args.count_chars,
    );
    if is_reading_stdin {
        println!("{:>8}", count);
    } else {
        println!("{:>8} {}", count, &args.filename);
    }

    Ok(())
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
