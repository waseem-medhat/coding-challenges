mod config;
mod scanner;

use crate::config::Config;
use crate::scanner::{ScanOutcome, with_port};

fn main() -> anyhow::Result<()> {
    let config = Config::from_args()?;

    match config {
        Config::Vanilla(_) => {
            println!("Scanning host(s): {:?}", config.hosts());
            scanner::vanilla(&config.hosts());
        }
        Config::SinglePort(_, port) => {
            println!("Scanning host(s): {:?} port: {}", config.hosts(), port);
            for outcome in with_port(&config.hosts(), port) {
                match outcome {
                    ScanOutcome::ResolveFailed(e) => println!("Host resolution failed: {e}"),
                    ScanOutcome::Open => println!("Port {} is open", port),
                    ScanOutcome::Closed => println!("Port {} is closed", port),
                    ScanOutcome::TimedOut => println!("Port {} timed out", port),
                    ScanOutcome::Unexpected(e) => println!("Unexpected err: {e}"),
                }
            }
        }
    }

    Ok(())
}
