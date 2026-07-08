//! Coding Challenges - `cat`
//!
//! - Challenge: <https://codingchallenges.fyi/challenges/challenge-cat>
//! - Status: **meets challenge requirements**
//!
//! - TODO
//!   - Input buffering
//!   - Non-printing chars (-vte)
//!   - Error handling and exit codes
//!
mod run;

use clap::Parser;
use run::run;

const LONG_ABOUT: &str = r#"A clone of the `cat` utility

The goal is to get close to feature parity while taking some liberties with the API or behavior, i.e., not meant to be an exact clone.

References:
- Coding Challenges: https://codingchallenges.fyi/challenges/challenge-cat
- cat: https://github.com/apple-oss-distributions/text_cmds/blob/main/cat/cat.c
"#;

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

    /// Acquire a lock on stdout
    #[arg(short = 'l')]
    lock_stdout: bool,

    /// Disable output buffering
    #[arg(short = 'u')]
    disable_buffering: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    run(args)
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::env;
    use std::fs::File;
    use std::io::{self, Write};

    #[test]
    fn nonexistent_file() -> anyhow::Result<()> {
        let mut cmd = Command::cargo_bin("cat")?;

        cmd.arg("nonexistent_file")
            .assert()
            .failure()
            .code(1)
            .stderr(predicates::str::contains("No such file"));

        Ok(())
    }

    #[test]
    fn basic_printing() -> anyhow::Result<()> {
        let text_path = env::temp_dir().join("basic.txt");
        let mut text = io::BufWriter::new(File::create(&text_path)?);
        for _ in 1..5 {
            writeln!(text, "hi")?;
        }

        let mut original_cmd = Command::new("cat");
        let expected_output = original_cmd.arg(&text_path).output()?;

        let mut cmd = Command::cargo_bin("cat")?;
        cmd.arg(&text_path)
            .assert()
            .success()
            .stdout(predicates::ord::eq(expected_output.stdout));

        Ok(())
    }

    #[test]
    fn line_numbers() -> anyhow::Result<()> {
        let text_path = env::temp_dir().join("nums.txt");
        let mut text = io::BufWriter::new(File::create(&text_path)?);
        for _ in 1..5 {
            writeln!(text, "lorem ipsum")?;
        }

        let mut original_cmd = Command::new("cat");
        let expected_output = original_cmd.arg("-n").arg(&text_path).output()?;

        let mut cmd = Command::cargo_bin("cat")?;
        cmd.arg(&text_path)
            .arg("-n")
            .assert()
            .success()
            .stdout(predicates::ord::eq(expected_output.stdout));

        Ok(())
    }

    #[test]
    fn nonblank_line_numbers() -> anyhow::Result<()> {
        let text_path = env::temp_dir().join("nonblank.txt");
        let mut text = io::BufWriter::new(File::create(&text_path)?);
        for _ in 1..5 {
            writeln!(text, "lorem ipsum")?;
        }

        let mut original_cmd = Command::new("cat");
        let expected_output = original_cmd.arg("-b").arg(&text_path).output()?;

        let mut cmd = Command::cargo_bin("cat")?;
        cmd.arg(&text_path)
            .arg("-b")
            .assert()
            .success()
            .stdout(predicates::ord::eq(expected_output.stdout));

        Ok(())
    }
}
