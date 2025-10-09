use std::{env, io, str::FromStr};

pub struct Config {
    host: String,
    port: u16,
}

impl Config {
    pub fn from_args() -> Result<Config, io::Error> {
        let mut args = env::args().skip(1);
        let mut host: Option<String> = None;
        let mut port: Option<u16> = None;

        loop {
            match args.next() {
                None => break,
                Some(arg) if arg.starts_with("-host=") => host = Some(get_arg_value(arg)?),
                Some(arg) if arg.starts_with("-port=") => port = Some(get_arg_value(arg)?),
                _ => return Err(err_invalid_input("no arguments provided")),
            }
        }

        Ok(Config {
            host: host.unwrap(),
            port: port.unwrap(),
        })
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> u16 {
        self.port.clone()
    }
}

fn get_arg_value<T: FromStr>(arg_string: String) -> Result<T, io::Error> {
    arg_string
        .split_once('=')
        .map(|(_, val)| String::from(val))
        .ok_or(err_invalid_input("no arguments provided"))
        .and_then(|val| {
            val.parse::<T>()
                .map_err(|_| err_invalid_input("couldn't parse arg"))
        })
}

fn err_invalid_input(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, msg)
}
