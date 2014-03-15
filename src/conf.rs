use std::io::net::ip::SocketAddr;

pub struct Config {
    debug : bool,
    bind_addr : SocketAddr,
}