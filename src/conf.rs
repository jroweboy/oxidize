use std::io::net::ip::SocketAddr;

pub struct Config {
    pub debug : bool,
    pub template_dir : Option<&'static str>,
    pub bind_addr : SocketAddr,
}