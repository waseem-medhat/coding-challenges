use std::env::{args, var};

fn main() {
    args().skip(1).for_each(|cmd| println!("{cmd}"));
    var("PATH")
        .expect("no PATH variable")
        .split(":")
        .for_each(|path| println!("{path}"));
}
