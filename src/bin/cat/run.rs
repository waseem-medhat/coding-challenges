use std::fs;
use std::io::{self, IsTerminal, Read, Write};

use anyhow::Context;

use crate::Args;

pub fn run(args: Args) -> anyhow::Result<()> {
    let content = if io::stdin().is_terminal() {
        read_file_content(&args.filenames)?
    } else {
        read_stdin_content()?
    };

    let stdout_lock = io::stdout().lock();
    let mut stdout_handle = io::BufWriter::new(stdout_lock);
    if args.print_nums_nonblank {
        return content
            .lines()
            .try_fold(1, |line_count, line| match line.is_empty() {
                false => writeln!(stdout_handle, " {:>3} {line}", line_count)
                    .map(|_| line_count + 1)
                    .with_context(|| "couldn't print!"),
                true => writeln!(stdout_handle, "     {line}")
                    .map(|_| line_count)
                    .with_context(|| "couldn't print!"),
            })
            .map(|_| ());
    }

    if args.print_nums {
        return content
            .lines()
            .enumerate()
            .try_for_each(|(i, line)| {
                writeln!(stdout_handle, " {:>3} {line}", i + 1).with_context(|| "couldn't print!")
            })
            .map(|_| ());
    }

    println!("{}", content);
    Ok(())
}

fn read_file_content(filenames: &[String]) -> anyhow::Result<String> {
    let mut content = String::new();
    for filename in filenames {
        content +=
            &fs::read_to_string(filename).with_context(|| format!("Couldn't read {}", filename))?;
    }
    Ok(content)
}

fn read_stdin_content() -> anyhow::Result<String> {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .with_context(|| "Couldn't read from stdin")?;
    Ok(content)
}

