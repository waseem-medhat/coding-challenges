use anyhow::anyhow;
use std::env;

pub enum Config {
    SinglePort(Vec<String>, u16),
    Vanilla(Vec<String>),
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
        match self {
            Self::SinglePort(hosts, _) => hosts.clone(),
            Self::Vanilla(hosts) => hosts.clone(),
        }
    }
}

fn build_config(hosts_arg: Option<Vec<String>>, port_arg: Option<u16>) -> anyhow::Result<Config> {
    match (hosts_arg, port_arg) {
        (Some(hosts), Some(port)) if hosts.len() == 1 => Ok(Config::SinglePort(hosts, port)),
        (Some(hosts), None) if hosts.len() == 1 => Ok(Config::Vanilla(hosts)),
        (None, _) => Err(anyhow!("host(s) must be provided")),
        _ => Err(anyhow!(
            "invalid inputs: currently only supporting single-host single-port scans and vanilla scans"
        )),
    }
}

fn parse_hosts(host_str: &str) -> Vec<String> {
    host_str.split(",").map(|s| s.to_string()).collect()
}
