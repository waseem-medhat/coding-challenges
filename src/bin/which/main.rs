use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let cmds: Vec<String> = env::args().skip(1).collect();

    let path_var = env::var("PATH").expect("no PATH variable");
    let cmd_results = walk_paths(path_var, &cmds);

    cmds.iter().for_each(|cmd| match cmd_results.get(cmd) {
        None => println!("{cmd} not found"),
        Some(path) => println!("{}", path.to_string_lossy()),
    });
}

fn walk_paths(path_var: String, cmds: &Vec<String>) -> HashMap<String, PathBuf> {
    let mut cmd_results: HashMap<String, PathBuf> = HashMap::new();
    let mut paths = path_var.split(":");

    loop {
        let path_opt = paths.next();
        if path_opt.is_none() || cmd_results.len() == cmds.len() {
            break;
        }

        let path = path_opt.unwrap().to_string();
        if let Some(cmd) = find_in_path(&path, &cmds) {
            cmd_results
                .entry(cmd.clone())
                .or_insert(PathBuf::from(path).join(cmd));
        }
    }

    return cmd_results;
}

fn find_in_path(path: &String, cmds: &Vec<String>) -> Option<String> {
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
