use std::fs;
use std::io::{self, Read};

use anyhow::Context;

/// Reads the contents of the given file name(s).
pub fn read_file_content(filenames: &[String]) -> anyhow::Result<String> {
    let mut content = String::new();
    for filename in filenames {
        content +=
            &fs::read_to_string(filename).with_context(|| format!("Couldn't read {}", filename))?;
    }
    Ok(content)
}

/// Reads the contents of stdin.
pub fn read_stdin_content() -> anyhow::Result<String> {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .with_context(|| "Couldn't read from stdin")?;
    Ok(content)
}
