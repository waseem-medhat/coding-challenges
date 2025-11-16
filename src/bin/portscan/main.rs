mod config;
mod scanner;

use crate::config::Config;

fn main() -> anyhow::Result<()> {
    let config = Config::from_args()?;
    print!("Scanning host: {}", config.host());

    match config.port() {
        None => scanner::vanilla(&config.host()),
        Some(port) => {
            println!(", port: {}", port);
            match scanner::with_port(&config.host(), port) {
                Ok(()) => println!("port open"),
                Err(err) => {
                    println!("{:?}", err);
                    println!("port not open");
                }
            }
        }
    }

    Ok(())
}
