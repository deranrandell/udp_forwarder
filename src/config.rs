use std::env;
use std::net::SocketAddr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid source address")]
    InvalidSourceAddr(#[source] std::net::AddrParseError),

    #[error("Invalid destination address")]
    InvalidDestinationAddr(#[source] std::net::AddrParseError),
}

#[derive(Debug)]
pub struct Config {
    pub source_addr: SocketAddr,
    pub destination_addr: SocketAddr,
}

pub fn load_config() -> Result<Config, ConfigError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <source_address> <destination_address>", args[0]);
        std::process::exit(1);
    }

    let source_addr: SocketAddr = args[1].parse().map_err(ConfigError::InvalidSourceAddr)?;
    let destination_addr: SocketAddr = args[2].parse().map_err(ConfigError::InvalidDestinationAddr)?;

    Ok(Config { source_addr, destination_addr })
}
