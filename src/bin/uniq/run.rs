use crate::Args;
use coding_challenges::helpers::{read_file_content, read_stdin_content};

pub fn run(args: Args) -> anyhow::Result<()> {
    let content = match args.input_file.as_str() {
        "-" => read_stdin_content()?,
        _ => read_file_content(&[args.input_file])?,
    };

    let mut lines = content.lines();
    let mut current_line = match lines.next() {
        None => return Ok(()),
        Some(first_line) => first_line,
    };
    let mut current_line_count = 1;

    lines.for_each(|line| {
        if line == current_line {
            current_line_count += 1;
            return;
        }
        print_line(current_line, current_line_count, args.count, args.repeated);
        current_line = line;
        current_line_count = 1;
    });

    print_line(current_line, current_line_count, args.count, args.repeated);
    Ok(())
}

fn print_line(line: &str, count: i32, print_count: bool, print_repeated: bool) {
    if print_repeated && count == 1 {
        return;
    }
    if print_count {
        print!("{:>4} ", count);
    }
    println!("{line}");
}
