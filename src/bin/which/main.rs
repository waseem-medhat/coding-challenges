//! Coding Challenges - `which`
//!
//! - Challenge: <https://codingchallenges.fyi/challenges/challenge-which>
//! - Status: **meets requirements**
//! - TODO:
//!   - Integration testing against `which`
//!   - Support -a arg: print all instances instead of first one
//!
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs, process};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Command(s) to find in PATH
    cmds: Vec<String>,

    /// Print nothing. Exit with 0 status if <cmd> is found, and 1 otherwise.
    #[arg(short = 's')]
    silent: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let path_var = env::var("PATH").expect("no PATH variable");
    let cmd_results = walk_paths(path_var, &args.cmds);

    if !args.silent {
        args.cmds.iter().for_each(|cmd| match cmd_results.get(cmd) {
            None => println!("{cmd} not found"),
            Some(path) => println!("{}", path.to_string_lossy()),
        });
    }

    if cmd_results.len() != args.cmds.len() {
        process::exit(1)
    }

    Ok(())
}

fn walk_paths(path_var: String, cmds: &[String]) -> HashMap<String, PathBuf> {
    let mut cmd_results: HashMap<String, PathBuf> = HashMap::new();
    let mut paths = path_var.split(":");

    loop {
        let path_opt = paths.next();
        if path_opt.is_none() || cmd_results.len() == cmds.len() {
            break;
        }

        let path = path_opt.unwrap().to_string();
        if let Some(cmd) = find_in_path(&path, cmds) {
            cmd_results
                .entry(cmd.clone())
                .or_insert(PathBuf::from(path).join(cmd));
        }
    }

    cmd_results
}

fn find_in_path(path: &String, cmds: &[String]) -> Option<String> {
    let dir = fs::read_dir(path);

    if dir.is_err() {
        return None;
    }

    dir.unwrap()
        .find_map(|entry| {
            let file_name = entry
                .as_ref()
                .unwrap()
                .path()
                .file_name()?
                .to_string_lossy()
                .to_string();

            cmds.iter().find(|c| **c == file_name)
        })
        .cloned()
}
