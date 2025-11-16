use anyhow::anyhow;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub fn with_port(host: &String, port: u16) -> anyhow::Result<()> {
    let addr = format!("{}:{}", host, port)
        .to_socket_addrs()?
        .next()
        .ok_or(anyhow!("addr parsing error"))?;
    TcpStream::connect_timeout(&addr, Duration::from_millis(600))?;
    Ok(())
}

pub fn vanilla(host: &String) {
    for port in 1..=65535 {
        if let Ok(()) = with_port(host, port) {
            println!("Port {} is open", port)
        }
    }
}
