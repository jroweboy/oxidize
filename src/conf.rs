//! `Config` is a simple struct that holds any server configuration details 
//! such as bind address and ssl information. Debug mode will someday produce error messages
//! in a method similar to Django error pages.

use std::io::net::ip::SocketAddr;

/// Defines the configuration of the application. This will eventually hold things such as 
/// SSL information for rust-http and any other things deemed good to have
pub struct Config {
    /// Currently does nothing, but I want it to eventually switch whether errors are rendered
    /// to the page or whether it should send an email to the web master
    pub debug : bool,
    /// Rust is moving away from the whole SocketAddr thing. TODO change this as well
    /// See https://github.com/chris-morgan/rust-http/pull/87 
    pub bind_addr : SocketAddr,
}