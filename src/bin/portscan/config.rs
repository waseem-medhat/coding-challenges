use anyhow::anyhow;
use std::{
    env,
    net::{IpAddr, ToSocketAddrs},
};

pub enum Config {
    SinglePort(Hosts, u16),
    Vanilla(Hosts),
}

pub enum Hosts {
    Names(Vec<String>),
    // Ips(Vec<IpAddr>),
}

impl Config {
    pub fn from_args() -> anyhow::Result<Config> {
        let args = env::args().skip(1);
        let mut hosts: Option<Vec<String>> = None;
        let mut port: Option<u16> = None;

        for arg in args {
            let (key, val) = arg
                .split_once('=')
                .ok_or(anyhow!("invalid arg format: {}", arg))?;

            match key {
                "-host" => hosts = Some(parse_hosts(val)),
                "-port" => port = Some(val.parse()?),
                _ => (),
            }
        }

        build_config(hosts, port)
    }

    pub fn hosts(&self) -> Vec<String> {
        let hosts = match self {
            Self::SinglePort(hosts, _) => hosts,
            Self::Vanilla(hosts) => hosts,
        };

        match hosts {
            Hosts::Names(names) => names.clone(),
        }
    }
}

fn build_config(hosts_arg: Option<Vec<String>>, port_arg: Option<u16>) -> anyhow::Result<Config> {
    match (hosts_arg, port_arg) {
        (Some(hosts), Some(port)) if hosts.len() == 1 => {
            Ok(Config::SinglePort(Hosts::Names(hosts), port))
        }
        (Some(hosts), None) if hosts.len() == 1 => Ok(Config::Vanilla(Hosts::Names(hosts))),
        (None, _) => Err(anyhow!("host(s) must be provided")),
        _ => Err(anyhow!(
            "invalid inputs: currently only supporting single-host single-port scans and vanilla scans"
        )),
    }
}

fn parse_hosts(host_str: &str) -> Vec<String> {
    host_str.split(",").map(|s| s.to_string()).collect()
}

fn parse_addr(addr_str: &str) -> anyhow::Result<IpAddr> {
    match addr_str.parse() {
        Ok(ip) => Ok(ip),
        Err(_) => {
            let host = addr_str.to_socket_addrs();
            match host {
                Err(_) => Err(anyhow!("cannot parse host 1 {}", addr_str)),
                Ok(mut sock_addrs) => match sock_addrs.next() {
                    None => Err(anyhow!("cannot parse host 2 {}", addr_str)),
                    Some(sock_addr) => Ok(sock_addr.ip()),
                },
            }
        }
    }
}
