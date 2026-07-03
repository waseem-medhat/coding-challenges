mod config;
mod scanner;

use std::time::Instant;

use crate::{
    config::Config,
    scanner::{ScanOutcome, with_port},
};

fn main() -> anyhow::Result<()> {
    let started_at = Instant::now();
    let config = Config::from_args()?;
    run(config)?;
    let elapsed = started_at.elapsed();
    println!("\nRan for {:?}", elapsed);
    Ok(())
}

pub fn run(config: Config) -> anyhow::Result<()> {
    let (hosts, ports, report_closed_ports) = match config {
        Config::Vanilla(hosts) => {
            println!("Scanning host(s): {} (vanilla)", hosts.join(","));
            (hosts, (1..=65535).collect(), false)
        }
        Config::SinglePort(hosts, port) => {
            println!("Scanning host(s): {} port: {}", hosts.join(","), port);
            (hosts, vec![port], true)
        }
    };

    for host in &hosts {
        for port in &ports {
            match with_port(host, *port) {
                ScanOutcome::ResolveFailed(e) => {
                    println!("Host resolution failed: {e}");
                    break;
                }
                ScanOutcome::Open => println!("Port {} is open", port),
                ScanOutcome::Closed if report_closed_ports => println!("Port {} is closed", port),
                ScanOutcome::Closed => continue,
                ScanOutcome::TimedOut => println!("Port {} timed out", port),
                ScanOutcome::Unexpected(e) => println!("Unexpected err: {e}"),
            }
        }
    }

    Ok(())
}
