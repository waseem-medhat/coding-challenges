mod config;

use crate::config::Config;

fn main() {
    let config = Config::from_args();
    println!("{:?}", config.field_num());
}
