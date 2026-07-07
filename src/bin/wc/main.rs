//! Coding Challenges - wc
//!
//! - Challenge: <https://codingchallenges.fyi/challenges/challenge-wc>
//! - Status: **meets requirements**
//! - TODO
//!   - Testing
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

    /// Count words
    #[arg(short = 'l')]
    count_lines: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    run(args)
}
