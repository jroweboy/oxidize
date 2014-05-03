//! `Config` is a simple struct that holds any server configuration details 
//! such as bind address and ssl information. Debug mode should produce error messages
//! in a method similar to Django error pages.

use std::io::net::ip::SocketAddr;

/// Defines the configuration of 
/// `debug` - At some point, I hope to use this to switch error handling to provide
/// useful error handling to the developer and then hiding details from a client
pub struct Config {
    pub debug : bool,
    pub bind_addr : SocketAddr,
}