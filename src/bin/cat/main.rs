//! Coding Challenges - `cat`
//!
//! - Challenge: <https://codingchallenges.fyi/challenges/challenge-cat>
//! - Status: **meets requirements**
//!
//! - TODO
//!   - Testing
//!   - Args for buffering (-u) and stdout lock (-l)
//!   - Non-printing chars (-vte)
//!
mod run;

use clap::Parser;
use run::run;

const LONG_ABOUT: &str = r#"A clone of the `cat` utility

The goal is to achieve feature parity while taking some liberties with the API or behavior, i.e., it's not meant to be an exact clone.

The project was initially guided by Coding Challenges: https://codingchallenges.fyi/challenges/challenge-cat
but then used `cat` itself as the reference.
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
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    run(args)
}
