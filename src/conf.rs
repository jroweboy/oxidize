use std::io::net::ip::SocketAddr;

pub struct Config {
    debug : bool,
    pub bind_addr : SocketAddr,
}