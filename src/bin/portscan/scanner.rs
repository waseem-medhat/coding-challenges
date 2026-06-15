use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;

pub fn with_port(host: IpAddr, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::new(host, port);
    TcpStream::connect_timeout(&addr, Duration::from_millis(600))?;
    Ok(())
}

pub fn vanilla(host: IpAddr) {
    for port in 1..=65535 {
        if let Ok(()) = with_port(host, port) {
            println!("Port {} is open", port)
        }
    }
}
