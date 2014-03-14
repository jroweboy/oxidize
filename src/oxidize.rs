// don't think this is used anymore
//extern crate extra;
// libpcre provides regexs for routing
extern crate pcre;
// holds references to HashMap
extern crate collections;
// need to pass the time info to 
extern crate time;
// needed for some Encodable stuff
extern crate serialize;
// templating is provided by rust-mustache
extern crate mustache;
// handles all the http stuff
extern crate http;
// used for holding the pcre struct in a mutable multithreaded way
extern crate sync;


// It turns out its real easy to reexport mods :D
// People that extern mod oxidize can use oxidize::reexported::mod;
pub use http::status;
pub use request::Request;
pub use conf::Config;
pub use http::server::ResponseWriter;
pub use http::headers::content_type::MediaType;

use http::server::{Server, ResponseWriter}; 
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};
use std::default::Default;

use conf::Config;
use route::Router;
use sync::Arc;
use std::cell::RefCell;

pub mod route;
pub mod renderer;
pub mod request;
pub mod conf;

#[deriving(Clone)]
pub struct Oxidize {
    conf: Arc<Config>,
    router: RefCell<~Router>,
    // TODO: Add these other fields
    // db : &'a DatabaseThingy,
    // middleware : 
    // template : &'a Template,
    // template_dir : Path, // should this be a part of Template?
    // add whatever other plugable things we wanna make
}

impl Oxidize {
    pub fn new(conf: Config, router: ~Router) -> Oxidize {
        Oxidize {
            conf: Arc::new(conf),
            router: RefCell::new(router),
        }
    }

    pub fn serve(self) {
        println!("Server is now running at {}", self.conf.get().bind_addr.to_str());
        self.serve_forever();
    }
}

impl Server for Oxidize {
    fn get_config(&self) -> http::server::Config {
        let bind_addr = self.conf.get().bind_addr;
        http::server::Config { bind_address: bind_addr }
    }

    fn handle_request(&self, req: &http::server::Request, response: &mut ResponseWriter) {
        response.headers.date = Some(time::now_utc());
        response.headers.server = Some(~"Oxidize/0.0.0 (Ubuntu)");

        // create a request object
        let path = match req.request_uri {
            AbsolutePath(ref i) => i.to_str(),
            AbsoluteUri(ref i) => i.to_str(),
            Authority(ref i) => i.to_str(),
            Star => ~"error" // ?
        };
        let test_method = match from_str("GET") {
            Some(m) => m,
            None => http::method::Get
        };
        let my_request = &mut request::Request {
            method: test_method, 
            uri: path,
            ..Default::default()
        };

        self.router.with(|r: &~Router| {r.route(my_request, response);});
    }
}