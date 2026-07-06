use std::fs;
use std::io::{self, BufWriter, IsTerminal, Read, Write, stdout};

use anyhow::Context;

use crate::Args;

pub fn run(args: Args) -> anyhow::Result<()> {
    let content = if io::stdin().is_terminal() {
        read_file_content(&args.filenames)?
    } else {
        read_stdin_content()?
    };

    let mut stdout = get_stdout_handle(args.lock_stdout, args.disable_buffering);
    if args.print_nums_nonblank {
        return content
            .lines()
            .try_fold(1, |line_count, line| match line.is_empty() {
                false => writeln!(stdout, " {:>3} {line}", line_count)
                    .map(|_| line_count + 1)
                    .with_context(|| "couldn't print!"),
                true => writeln!(stdout, "     {line}")
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
                writeln!(stdout, " {:>3} {line}", i + 1).with_context(|| "couldn't print!")
            })
            .map(|_| ());
    }

    print!("{}", content);
    Ok(())
}

fn get_stdout_handle(lock_stdout: bool, disable_buffering: bool) -> Box<dyn Write> {
    match (lock_stdout, disable_buffering) {
        (true, false) => Box::new(BufWriter::new(stdout().lock())),
        (false, false) => Box::new(BufWriter::new(stdout())),
        (true, true) => Box::new(stdout().lock()),
        (false, true) => Box::new(stdout()),
    }
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
