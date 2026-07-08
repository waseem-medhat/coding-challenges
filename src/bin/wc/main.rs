//! Coding Challenges - wc
//!
//! - Challenge: <https://codingchallenges.fyi/challenges/challenge-wc>
//! - Status: **meets requirements**
//! - TODO
//!   - Support multiple files
//!   - Support -L arg
//!
mod run;

use clap::Parser;
use run::run;

#[derive(Parser)]
struct Args {
    filename: String,

    /// Count bytes
    #[arg(short = 'c')]
    count_bytes: bool,

    /// Count characters
    #[arg(short = 'm')]
    count_chars: bool,

    /// Count words
    #[arg(short = 'w')]
    count_words: bool,

    /// Count lines
    #[arg(short = 'l')]
    count_lines: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    run(args)
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::io::{self, Write};
    use std::{env, fs::File};

    #[test]
    fn nonexistent_file() -> anyhow::Result<()> {
        let text_path = env::temp_dir().join("nonexistent_file");

        Command::cargo_bin("wc")?
            .arg(&text_path)
            .assert()
            .failure()
            .code(1)
            .stderr(predicates::str::contains("No such file"));

        Ok(())
    }

    #[test]
    fn default_output() -> anyhow::Result<()> {
        let text_path = env::temp_dir().join("default.txt");
        let mut file = io::BufWriter::new(File::create(&text_path)?);
        for _ in 0..2 {
            writeln!(file, "lorém ipsum")?
        }
        file.flush()?;

        let expected_output = Command::new("wc").arg(&text_path).output()?.stdout;

        Command::cargo_bin("wc")?
            .arg(&text_path)
            .assert()
            .success()
            .stdout(expected_output);

        Ok(())
    }

    #[test]
    fn individual_counts() -> anyhow::Result<()> {
        let text_path = env::temp_dir().join("lines.txt");
        let mut file = io::BufWriter::new(File::create(&text_path)?);
        for _ in 0..2 {
            writeln!(file, "lorém ipsum")?
        }
        file.flush()?;

        for arg in ["-l", "-w", "-c", "-m"] {
            let expected_output = Command::new("wc").arg(arg).arg(&text_path).output()?.stdout;
            let expected_predicate = predicates::str::diff(String::from_utf8(expected_output)?);

            Command::cargo_bin("wc")?
                .arg(arg)
                .arg(&text_path)
                .assert()
                .success()
                .stdout(expected_predicate);
        }

        Ok(())
    }
}
