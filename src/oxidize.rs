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
// pub use response::Response;
pub use route::Route;
pub use http::server::ResponseWriter;

// I'm explicitly including http::server::Request right now
// since I have a struct also named Request
use http::server::{Config, Server, ResponseWriter}; 
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};
use http::headers;
use http::status;
use http::status::Status;
use collections::enum_set::{EnumSet};
use collections::hashmap::HashMap;
use pcre::{CompileOption, StudyOption, ExtraOption, Pcre};

use std::cast;
use std::default::Default;
use std::io::net::ip::{SocketAddr, Ipv4Addr};

use response::Response;
use conf::Config;
use route::{Route};
use sync::RWArc;


pub mod route;
pub mod renderer;
pub mod response;
pub mod request;
pub mod conf;

// initialize it to nothing 
// static mut compiled_routes : Option<Pcre> = None;
// maybe make a map from function pointer as the hash and the value be the 
// this checks out below. I can use http://static.rust-lang.org/doc/master/std/cast/fn.transmute.html
// to convert the fn pointer into a raw *() pointer (similar to a void pointer)
// which already has the Hash trait implemented. The reverse function will take in 
// a function pointer and context and return a url? I'm not confident about that one...

// Except for you evil little one. 
// TODO: how can I possibly expose a reverse function without static?
// static mut reverse_routes_str : Option<()> = None;
// HashMap::<*(), &'static str>::new();

#[deriving(Clone)]
pub struct Oxidize {
    // TODO: use this little piece of awesome I found to allow them to choose port and stuff
    //from_str::<SocketAddr>("127.0.0.1:8080").unwrap()
    // http://static.rust-lang.org/doc/0.9/std/io/index.html Found here
    conf: Config,

    // routes: &'static [Route<'static>],
    // compiled_routes: RWArc<Pcre>
}

impl Oxidize {

    pub fn new(conf: Config, r : &'static [Route<'static>]) -> Oxidize {
        Oxidize {
            conf : conf,
            // routes : r,
            // compiled_routes : Oxidize::compile_routes(r)
        }
    }

    // The Server trait has serve_forever as private, so this is my hackish way to expose it
    // TODO maybe ask upstream to change that?
    pub fn serve(self) {
        // let addr = self.get_config().bind_address;
        // println!("Server is now running at {}", addr.to_str());
        self.serve_forever();
    }
}

#[allow(unused_must_use)]
impl Server for Oxidize {

    fn get_config(&self) -> http::server::Config {
        // TODO: Read the data and better handle user data (see the struct def)
        http::server::Config { bind_address: self.conf.bind_addr }
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

        self.conf.router.route(my_request, response);
        // let router = self.conf.router;
        // let response_body = router.route(my_request,res);

        // res.headers.content_type = Some(headers::content_type::MediaType {
        //     type_: ~"text",
        //     subtype: ~"html",
        //     parameters: ~[]
        // });

        // res.headers.content_length = Some(response_body.len());
        // res.write(response_body.as_bytes());
    }
}