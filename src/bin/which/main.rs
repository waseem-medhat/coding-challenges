use std::collections::HashMap;
use std::env::{args, var};
use std::fs;
use std::path::PathBuf;

fn main() {
    let cmds: Vec<String> = args().skip(1).collect();
    let mut cmd_results: HashMap<String, PathBuf> = HashMap::new();

    var("PATH")
        .expect("no PATH variable")
        .split(":")
        .for_each(|path| {
            if let Some(cmd) = find_cmd(String::from(path), &cmds) {
                cmd_results
                    .entry(cmd.clone())
                    .or_insert(PathBuf::from(path).join(cmd));
            }
        });

    cmds.iter().for_each(|cmd| match cmd_results.get(cmd) {
        None => println!("{cmd} not found"),
        Some(path) => println!("{}", PathBuf::from(path).join(cmd).to_string_lossy()),
    });
}

fn find_cmd(path: String, cmds: &Vec<String>) -> Option<String> {
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
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            cmds.iter().find(|c| **c == file_name)
        })
        .cloned()
}
