use std::{
    io::ErrorKind,
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

use anyhow::{Context, anyhow};

pub enum ScanOutcome {
    Open,
    Closed,
    TimedOut,
    ResolveFailed(anyhow::Error),
    Unexpected(anyhow::Error),
}

pub fn with_port(host: &[String], port: u16) -> Vec<ScanOutcome> {
    host.iter()
        .map(|host| match resolve(host, port) {
            Err(e) => ScanOutcome::ResolveFailed(e),
            Ok(socket_addr) => {
                match TcpStream::connect_timeout(&socket_addr, Duration::from_millis(600)) {
                    Ok(_) => ScanOutcome::Open,
                    Err(e) => match e.kind() {
                        ErrorKind::TimedOut => ScanOutcome::TimedOut,
                        ErrorKind::ConnectionRefused => ScanOutcome::Closed,
                        _ => ScanOutcome::Unexpected(e.into()),
                    },
                }
            }
        })
        .collect()
}

pub fn vanilla(hosts: &[String]) {
    'vanilla: for port in 1..=65535 {
        for outcome in with_port(hosts, port) {
            match outcome {
                ScanOutcome::ResolveFailed(e) => {
                    println!("Host resolution failed: {e}");
                    break 'vanilla;
                }
                ScanOutcome::Open => println!("Port {} is open", port),
                ScanOutcome::Closed => continue,
                ScanOutcome::TimedOut => println!("Port {} timed out", port),
                ScanOutcome::Unexpected(e) => println!("Unexpected err: {e}"),
            }
        }
    }
}

fn resolve(host: &str, port: u16) -> anyhow::Result<SocketAddr> {
    format!("{}:{}", host, port)
        .to_socket_addrs()
        .with_context(|| "couldn't resolve {host} to socket addr")?
        .next()
        .ok_or(anyhow!("resolving {host} yielded zero addrs"))
}
