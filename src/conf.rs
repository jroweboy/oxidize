use std::io::net::ip::SocketAddr;
use route::Router;
// use renderer::Template;

#[deriving(Clone)]
pub struct Config {
    debug : bool,
    bind_addr : SocketAddr,
    // bind_port : uint,
    router : ~Router,
    // TODO: Add these other fields
    // db : &'a DatabaseThingy,
    // middleware : 
    // template : &'a Template,
    // template_dir : Path, // should this be a part of Template?
    // add whatever other plugable things we wanna make
}