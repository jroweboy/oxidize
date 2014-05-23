// TODO define all the lib things and linking stuff up here

// the glorious logging crate
#![feature(phase)]
#[phase(syntax, link)] extern crate log;
// holds references to HashMap
extern crate collections;
// need to pass the time info to 
extern crate time;
// needed for some Encodable stuff
extern crate serialize;
// templating is provided by rust-mustache -- making that required in userland only!
// extern crate mustache;
// handles all the http stuff
extern crate http;
// used for holding the pcre struct in a mutable multithreaded way
extern crate sync;
//user for initializeing the url
extern crate url;

// It turns out its real easy to reexport mods :D
// People that extern mod oxidize can use oxidize::reexported::mod;
pub use http::status;
pub use request::Request;
pub use conf::Config;
pub use http::server::ResponseWriter;
pub use http::headers::content_type::MediaType;

use http::server::{Server, ResponseWriter}; 
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};

use conf::Config;
use route::Router;
use sync::Arc;
use std::cell::RefCell;

pub mod route;
pub mod request;
pub mod conf;

#[deriving(Clone)]
pub struct Oxidize {
    conf: Arc<Config>,
    router: RefCell<~Router:Send>,
}

impl Oxidize {
    pub fn new(conf: Config, router: ~Router:Send) -> Oxidize {
        Oxidize {
            conf: Arc::new(conf),
            router: RefCell::new(router),
        }
    }

    pub fn serve(self) {
        debug!("Server is now running at {}", self.conf.bind_addr.to_str());
        self.serve_forever();
    }

}

impl Server for Oxidize {
    fn get_config(&self) -> http::server::Config {
        let bind_addr = self.conf.bind_addr;
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
        // This tries to convert the path to a Url struct
        let url: url::Url = match url::from_str(path) {
            Ok(m) => m,
            Err(partial_url) => {
                //Partial paths are not supported yet, see
                //https://github.com/mozilla/rust/issues/10706
                let new_path = ~"http://127.0.0.1" + path;
                println!("Was {}, now is {}", path, new_path);
                match url::from_str(new_path) {
                    Ok(n) => n,
                    Err(bad_url) => {
                        info!("Bad or Partial Url found: {}", bad_url);
                        url::Url {  
                            scheme: ~"",
                            user: None,
                            host: ~"",
                            port: None,
                            path: ~"",
                            query: Vec::new(),
                            fragment: None }
                    }
                }
            }
        };
        let my_request = &mut request::Request {
            method: test_method, 
            uri: ~url.clone(),
            GET : None,
            POST : None,
            context : None,
            router : self.router.borrow()
        };

        // self.router.with(|r: &~Router| {r.route(my_request, response);});
        self.router.borrow().route(my_request, response);
    }
}
