mod config;
mod scanner;

use crate::config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_args()?;
    print!("Scanning host: {}", config.host());

    match config.port() {
        None => scanner::vanilla(&config.host()),
        Some(port) => {
            println!(", port: {}", port);
            if scanner::with_port(&config.host(), port) {
                println!("port open")
            } else {
                println!("port not open")
            }
        }
    }

    Ok(())
}
