#![feature(phase)]
#[phase(syntax, link)] extern crate log;

extern crate http;
extern crate sync;
use std::io::net::ip::SocketAddr;
use std::vec::Vec;
use std::cell::RefCell;
use http::server::ResponseWriter;
use http::server::Server;

use sync::{RWLock, Arc, Weak};

#[deriving(Clone)]
struct Oxidize<'a> {
    pub conf : Arc<Config>,
    pub router : RefCell<~Router<'a>:Send+Share>,
    // pub filters : ~[~MiddleWare],
}

trait App<'a>:Send+Share {
    fn get_router(&'a self) -> ~Router<'a>:Send+Share;
    fn handle_route(&self, reverse: &str);
}

impl<'a> App<'a> for MyApp<'a> {
    fn get_router(&'a self) -> ~Router<'a>:Send+Share {
        ~MatchRouter {
            app: Arc::new(RWLock::new(self as &'a App:Send+Share)),
            routes: Arc::new(RWLock::new(~[
                ("GET", "index", "/index"), 
                ("GET", "hello", "/hello"), 
            ])),
        } as ~Router:Send+Share
    }

    fn handle_route(&self, reverse: &str) {
        match reverse {
            "/index" => self.index(),
            "/hello" => self.hello(),
            _ => unreachable!(),
        }
    }
}

struct MyApp<'a> {
    router : Weak<&'a Router<'a>:Send+Share>,
    // renderer: Arc<Renderer>,
}

trait Router<'a>:Send {
    fn add_route(&self, method: &str, name: &str, url: &str);
    fn route(&self, Request, &mut Response);
    fn copy(&self) -> ~Router<'a>:Send+Share;
}

impl<'a> Clone for ~Router<'a>:Send+Share {
    fn clone(&self) -> ~Router<'a>:Send+Share {
        self.copy()
    }
}

// not fully operational but good enough for now
struct MatchRouter<'a> {
    app : Arc<RWLock<&'a App<'a>:Send+Share>>,
    routes : Arc<RWLock<~[(&'static str, &'static str, &'static str)]>>,
}

impl<'a> Router<'a> for MatchRouter<'a> {
    fn add_route(&self, method: &str, name: &str, url: &str){
        // self.routes.write().push(f);
    }
    fn route(&self, req: Request, res: &mut Response) {
        match req.url {
            "/index" | "/hello" => self.app.read().handle_route(req.url),
            _ => unreachable!(),
        };
    }
    fn copy(&self) -> ~Router<'a>:Send+Share {
        ~MatchRouter {
            app: self.app.clone(),
            routes: self.routes.clone(),
        } as ~Router<'a>:Send+Share
    }
}

impl<'a> MyApp<'a> {
    fn index(&self) { println!("index!"); } //self.renderer.print(); }
    fn hello(&self) { println!("hello!"); }
}


impl<'a> Server for Oxidize<'a> {
    fn get_config(&self) -> http::server::Config {
        let bind_addr = self.conf.bind_addr;
        http::server::Config { bind_address: bind_addr }
    }

    fn handle_request(&self, req: &http::server::Request, response: &mut ResponseWriter) {
        let q = Request{ url: "test" };
        let mut s = Response{ text: ~"hehe" };

        self.router.borrow().route(q, &mut s);
    }
}

impl<'a> Oxidize<'a> {
    fn serve(self) {
        println!("Server is now running at {}", self.conf.bind_addr.to_str());
        self.serve_forever();
    }
}
fn main() {
    
    let conf = Config {
        bind_addr: from_str::<SocketAddr>("127.0.0.1:8001").unwrap(),
    };
}



struct Renderer;

impl Renderer {
    fn print(&self) {
        println!("Rendered!");
    }
}

struct Config {
    pub bind_addr : SocketAddr,
}

struct Request<'a> {
    url: &'a str,
}

struct Response {
    text: ~str
}