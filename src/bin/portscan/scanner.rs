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

pub fn with_port(host: &str, port: u16) -> ScanOutcome {
    match resolve(host, port) {
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
    }
}

fn resolve(host: &str, port: u16) -> anyhow::Result<SocketAddr> {
    format!("{}:{}", host, port)
        .to_socket_addrs()
        .with_context(|| format!("couldn't resolve {host} to socket addr"))?
        .next()
        .ok_or(anyhow!("resolving {host} yielded zero addrs"))
}
