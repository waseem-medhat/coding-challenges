use anyhow::anyhow;
use std::env;

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
            let (key, val) = arg
                .split_once('=')
                .ok_or(anyhow!("invalid arg format: {}", arg))?;

            match key {
                "-host" => host = Some(val.parse()?),
                "-port" => port = Some(val.parse()?),
                _ => (),
            }
        }

        Ok(Config {
            host: host.ok_or(anyhow!("host must be provided"))?,
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
