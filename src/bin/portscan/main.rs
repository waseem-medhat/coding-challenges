mod config;
mod scanner;

use crate::config::Config;

fn main() -> anyhow::Result<()> {
    let config = Config::from_args()?;

    match config {
        Config::Vanilla(_) => {
            println!("Scanning host(s): {:?}", config.hosts());
            scanner::vanilla(&config.hosts());
        }
        Config::SinglePort(_, port) => {
            println!("Scanning host(s): {:?} port: {}", config.hosts(), port);
            match scanner::with_port(&config.hosts(), port) {
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
