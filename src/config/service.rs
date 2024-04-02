use std::net::IpAddr;
use std::net::{AddrParseError, SocketAddr};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceConfig {
    pub host: IpAddr,
    pub port: u16,
}

impl ServiceConfig {
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_socket_addr(&self) -> Result<SocketAddr, AddrParseError> {
        self.get_addr().parse()
    }
}
