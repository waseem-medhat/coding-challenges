mod config;
mod scanner;

use crate::config::Config;

fn main() -> anyhow::Result<()> {
    let config = Config::from_args()?;
    print!("Scanning host(s): {:?}", config.hosts());

    match config {
        Config::Vanilla(host) => scanner::vanilla(host),
        Config::Single(host, port) => {
            println!(", port: {}", port);
            match scanner::with_port(host, port) {
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
