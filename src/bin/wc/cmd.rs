use crate::config::CountOpt;

pub fn count(content: &String, count_opt: &CountOpt) -> String {
    match count_opt {
        CountOpt::Bytes => count_bytes(content).to_string(),
        CountOpt::Lines => count_lines(content).to_string(),
        CountOpt::Words => count_words(content).to_string(),
        CountOpt::Chars => count_chars(content).to_string(),
        CountOpt::All => {
            let bytes = count_bytes(content).to_string();
            let words = count_words(content).to_string();
            let lines = count_lines(content).to_string();

            format!(" {:>7} {:>7} {:>7}", lines, words, bytes)
        }
    }
}

fn count_bytes(content: &String) -> usize {
    content.len()
}

fn count_lines(content: &String) -> usize {
    content.chars().filter(|&c| c == '\n').count()
}

fn count_words(content: &String) -> usize {
    content.split_ascii_whitespace().count()
}

fn count_chars(content: &String) -> usize {
    content.chars().count()
}
