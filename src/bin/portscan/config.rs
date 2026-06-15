use anyhow::{Ok, anyhow};
use std::{env, net::IpAddr};

// pub struct Config {
//     hosts: Vec<IpAddr>,
//     port: Option<u16>,
// }

pub enum Config {
    Single(IpAddr, u16),
    Vanilla(IpAddr),
}

impl Config {
    pub fn from_args() -> anyhow::Result<Config> {
        let args = env::args().skip(1);
        let mut hosts: Option<Vec<IpAddr>> = None;
        let mut port: Option<u16> = None;

        for arg in args {
            let (key, val) = arg
                .split_once('=')
                .ok_or(anyhow!("invalid arg format: {}", arg))?;

            match key {
                "-host" => hosts = Some(parse_hosts(val)?),
                "-port" => port = Some(val.parse()?),
                _ => (),
            }
        }

        Ok(build_config(hosts, port)?)
    }

    pub fn hosts(&self) -> Vec<IpAddr> {
        match self {
            Self::Single(host, _) => vec![*host],
            Self::Vanilla(host) => vec![*host],
        }
    }
}

fn build_config(hosts: Option<Vec<IpAddr>>, port: Option<u16>) -> anyhow::Result<Config> {
    match (hosts, port) {
        (Some(addrs), Some(port)) if addrs.len() == 1 => Ok(Config::Single(addrs[0], port)),
        (Some(addrs), None) if addrs.len() == 1 => Ok(Config::Vanilla(addrs[0])),
        (None, _) => Err(anyhow!("host(s) must be provided")),
        _ => Err(anyhow!(
            "invalid inputs: currently only supporting single-host single-port scans and vanilla scans"
        )),
    }
}

fn parse_hosts(host_str: &str) -> anyhow::Result<Vec<IpAddr>> {
    host_str.split(",").map(parse_addr).collect()
}

fn parse_addr(addr_str: &str) -> anyhow::Result<IpAddr> {
    addr_str
        .parse()
        .map_err(|err| anyhow!("addr parse error with {}: {}", addr_str, err))
}
