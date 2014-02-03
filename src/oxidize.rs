extern mod http;
extern mod extra;

// It turns out its real easy to reexport mods :D
// People that extern mod oxidize can use oxidize::reexported::mod;
pub use http::status;
pub use http::server::Request;
pub use response::Response;
pub use route::Route;

// I'm explicitly including http::server::Request right now
// since I have a struct also named Request
use http::server::{Config, Server, ResponseWriter}; 
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};
use http::headers;
use http::status;
use http::status::Status;

use extra::time;
use std::io::net::ip::{SocketAddr, Ipv4Addr};

use response::Response;
use route::Route;

pub mod route;
pub mod renderer;
pub mod response;
pub mod request;

#[deriving(Clone)]
pub struct Oxidize {
    // TODO: use this little piece of awesome I found to allow them to choose port and stuff
    //from_str::<SocketAddr>("127.0.0.1:8080").unwrap()
    // http://static.rust-lang.org/doc/0.9/std/io/index.html Found here
    port : u16,
    addr : ~str,
    routes : &'static [Route<'static>]
    // TODO: Maybe add the templates dir as an conf option?
}

impl Oxidize {

    pub fn new(p : u16, a : &str, r : &'static [Route<'static>]) -> Oxidize {
        Oxidize {
            port : p,
            addr : a.to_owned(),
            routes : r
        }
    }

    // TODO: Shouldn't route just take in a request?
    fn route(&self, request: &http::server::Request, response: &mut ResponseWriter) -> ~str {
        let path = match request.request_uri {
            AbsolutePath(ref i) => i.to_str(),
            AbsoluteUri(ref i) => i.to_str(),
            Authority(ref i) => i.to_str(),
            Star => ~"error" // ?
        };
        println!("Path: {}", path);
        let method = request.method.to_str();

        // maybe if we are cool enough we can use the router regexp from revel
        // https://github.com/robfig/revel/blob/master/router.go#L305
        let mut resp : Option<~Response> = None;
        // I bet there is some zip magic that COULD be done here to clean this bit up
        // but since we wanna add in varabile routes I don't think we should bother with that
        for route in self.routes.iter() {
            if method == route.method.to_owned() {
                // TODO: fancy magic required here to match variable routes to the given routes
                if path == route.path.to_owned() {
                    resp = Some(route.call(request));
                }
            }
        }
        let res : ~Response = match resp {
            Some(res) => res,
            None => ~Response {status: status::NotFound, content: ~"Not Found"}
        };

        let reason = res.status.reason();
        let code = res.status.code();

        let newStatus = Status::from_code_and_reason(code,reason);

        response.status = newStatus;
        return res.content;
    }

    // The Server trait has serve_forever as private, so this is my hackish way to expose it
    pub fn serve(self) {
        self.serve_forever();
    }
}

impl Server for Oxidize {

    fn get_config(&self) -> Config {
        // TODO: Read the data from the self.addr and self.port lol
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: self.port } }
    }

    fn handle_request(&self, req: &http::server::Request, res: &mut ResponseWriter) {
        res.headers.date = Some(time::now_utc());
        res.headers.server = Some(~"Oxidize/0.0.0 (Ubuntu)");

        // TODO: send route the oxidize::Request instead of the http::server::Request
        // if we use our own Request struct then we can add whatever we want to it
        let response_body = self.route(req,res);

        res.headers.content_type = Some(headers::content_type::MediaType {
            type_: ~"text",
            subtype: ~"html",
            parameters: ~[]
        });

        res.headers.content_length = Some(response_body.len());
        //debug!("Response: {}", response_body);
        res.write(response_body.as_bytes());
    }
}