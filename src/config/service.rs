use std::net::IpAddr;
use std::net::{AddrParseError, SocketAddr};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Service {
    pub host: IpAddr,
    pub port: u16,
    pub prefix: String,
}

impl Service {
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_socket_addr(&self) -> Result<SocketAddr, AddrParseError> {
        self.get_addr().parse()
    }
}
