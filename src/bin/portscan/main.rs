mod config;

use crate::config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_args()?;

    println!("{}", config.host());
    println!("{}", config.port());

    Ok(())
}
