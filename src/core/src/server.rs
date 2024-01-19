use std::net::SocketAddr;
use crate::constants::{DEFAULT_DB_NAME, DEFAULT_IP, DEFAULT_PORT};

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub socket_addr: SocketAddr,
    pub db_name: String,
}

impl Default for ConnectionInfo {
    fn default() -> Self {
        Self {
            socket_addr: SocketAddr::new(DEFAULT_IP.parse().unwrap(), DEFAULT_PORT),
            db_name: DEFAULT_DB_NAME.to_string(),
        }
    }
}

impl ConnectionInfo {
    pub fn new(ip: String, port: u16) -> Self {
        Self {
            socket_addr: SocketAddr::new(ip.parse().unwrap(), port),
            db_name: DEFAULT_DB_NAME.to_string(),
        }
    }

    pub fn get_addr(&self) -> String {
        format!("{}", self.socket_addr)
    }
}