use anyhow::anyhow;
use std::{env, str::FromStr};

pub struct Config {
    host: String,
    port: Option<u16>,
}

impl Config {
    pub fn from_args() -> anyhow::Result<Config> {
        let args = env::args().skip(1);
        let mut host: Option<String> = None;
        let mut port: Option<u16> = None;

        for arg in args {
            if arg.starts_with("-host=") {
                host = Some(get_arg_value(&arg)?)
            }
            if arg.starts_with("-port=") {
                port = Some(get_arg_value(&arg)?)
            }
        }

        if host.is_none() {
            return Err(anyhow!("host must be specified"));
        }

        Ok(Config {
            host: host.expect(""),
            port,
        })
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }
}

fn get_arg_value<T: FromStr>(arg_string: &str) -> anyhow::Result<T> {
    let (_, val) = arg_string
        .split_once('=')
        .ok_or(anyhow!("invalid arg format"))?;

    val.parse::<T>().map_err(|_| anyhow!("arg parsing error"))
}
