use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use anyhow::anyhow;

pub fn with_port(hosts: &Vec<String>, port: u16) -> anyhow::Result<()> {
    for host in hosts {
        let socket_addr = format!("{}:{}", host, port)
            .to_socket_addrs()?
            .next()
            .ok_or(anyhow!("bad addr??"))?;

        TcpStream::connect_timeout(&socket_addr, Duration::from_millis(600))?;
    }
    Ok(())
}

pub fn vanilla(hosts: &Vec<String>) {
    for port in 1..=65535 {
        if let Ok(()) = with_port(hosts, port) {
            println!("Port {} is open", port)
        }
    }
}
