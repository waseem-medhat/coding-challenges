//! # Coding Challenges - uniq
//!
//! - Challenge: <https://codingchallenges.fyi/challenges/challenge-uniq>
//! - Status: **matches challenge requirements**
//! - TODO
//!   - Feature parity with original tool
//!
use clap::Parser;

mod run;

use run::run;

#[derive(Parser)]
struct Args {
    /// Path of file from which content is read. If it's '-' or empty, will read from stdin.
    #[arg(default_value = "-")]
    input_file: String,

    /// Path of file to which unique lines are written. If it's empty, will write to stdout.
    output_file: Option<String>,

    /// Count lines.
    #[arg(short, long)]
    count: bool,

    /// Print lines that have duplicates.
    #[arg(short = 'd', long)]
    repeated: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    run(args)
}

#[cfg(test)]
mod tests {
    use std::io::{BufWriter, Write};
    use std::{env::temp_dir, fs::File};

    use assert_cmd::Command;

    const TEST_CONTENT: &str = "line1
line2
line2
line3
line4";

    #[test]
    fn file_input() -> anyhow::Result<()> {
        let file_path = temp_dir().join("uniq_file_input.txt");
        let mut file = BufWriter::new(File::create(&file_path)?);
        writeln!(file, "{TEST_CONTENT}")?;
        file.flush()?;

        let expected_stdout = Command::new("uniq").arg(&file_path).output()?.stdout;
        let expected_predicate = predicates::str::diff(String::from_utf8(expected_stdout)?);

        Command::cargo_bin("uniq")?
            .arg(&file_path)
            .assert()
            .success()
            .stdout(expected_predicate);

        Ok(())
    }

    #[test]
    fn count() -> anyhow::Result<()> {
        let file_path = temp_dir().join("uniq_count.txt");
        let mut file = BufWriter::new(File::create(&file_path)?);
        writeln!(file, "{TEST_CONTENT}")?;
        file.flush()?;

        let expected_stdout = Command::new("uniq")
            .arg("-c")
            .arg(&file_path)
            .output()?
            .stdout;
        let expected_predicate = predicates::str::diff(String::from_utf8(expected_stdout)?);

        Command::cargo_bin("uniq")?
            .arg("-c")
            .arg(&file_path)
            .assert()
            .success()
            .stdout(expected_predicate);

        Ok(())
    }

    #[test]
    fn repeated() -> anyhow::Result<()> {
        let file_path = temp_dir().join("uniq_count.txt");
        let mut file = BufWriter::new(File::create(&file_path)?);
        writeln!(file, "{TEST_CONTENT}")?;
        file.flush()?;

        let expected_stdout = Command::new("uniq")
            .arg("-d")
            .arg(&file_path)
            .output()?
            .stdout;
        let expected_predicate = predicates::str::diff(String::from_utf8(expected_stdout)?);

        Command::cargo_bin("uniq")?
            .arg("-d")
            .arg(&file_path)
            .assert()
            .success()
            .stdout(expected_predicate);

        Ok(())
    }
}
