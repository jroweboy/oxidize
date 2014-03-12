use route::{Router};
use sync::RWArc;
use std::io::net::ip::SocketAddr;

pub struct Config {
    debug : bool,
    bind_addr : SocketAddr,
}