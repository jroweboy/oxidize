use std::io::net::ip::SocketAddr;

pub struct Config {
    pub debug : bool,
    pub bind_addr : SocketAddr,
}