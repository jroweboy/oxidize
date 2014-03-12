use route::{Router};
use sync::Arc;
use std::io::net::ip::SocketAddr;

pub struct Config {
    debug : bool,
    bind_addr : SocketAddr,
    // routes : &'static [~Route<'static>:Send+Freeze],
    router : Arc<~Router:Send+Freeze>,
    // TODO: Add these other fields
    // db : &'a DatabaseThingy,
    // middleware : 
    // template : &'a Template,
    // template_dir : Path, // should this be a part of Template?
    // add whatever other plugable things we wanna make
}

// impl Config {
//     pub fn new(debug: bool, 