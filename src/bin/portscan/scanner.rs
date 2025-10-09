use std::net::TcpStream;

pub fn with_port(host: &String, port: u16) -> bool {
    TcpStream::connect(format!("{}:{}", host, port)).is_ok()
}

pub fn vanilla(host: &String) {
    for port in 1..=65535 {
        if with_port(host, port) {
            println!("Port {} is open", port)
        }
    }
}
