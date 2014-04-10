#![feature(phase)]
#[phase(syntax, link)] extern crate log;

extern crate http;
extern crate sync;
use std::io::net::ip::SocketAddr;
use std::vec::Vec;
use std::cell::RefCell;
use http::server::ResponseWriter;
use http::server::Server;

use sync::{RWLock, Arc};

#[deriving(Clone)]
struct Oxidize<'a> {
    pub config : &'a Config,
    // pub app : &'a App,
    pub router : RefCell<~Router:Send>,
    // pub filters : ~[~MiddleWare],
}

trait App {
    fn get_routes(&self) -> ~Router:Send;
}

struct MyApp {
    pub render: Renderer,
}

struct Renderer;

impl Renderer {
    fn print() {
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

trait Router:Send {
    fn add_route(&self, &str, |Request, &mut Response|);
    fn route(&self, Request, &mut Response);
    fn copy(&self) -> ~Router:Send;
}

impl Clone for ~Router:Send {
    fn clone(&self) -> ~Router:Send {
        self.copy()
    }
}

// not fully operational but good enough for now
struct MatchRouter<'a> {
    routes : Arc<RWLock<Vec<|Request, &mut Response|: 'a>>>,
}

impl<'a> Router for MatchRouter<'a> {
    fn add_route(&self, path: &str, f: |Request, &mut Response|: 'a){
        self.routes.write().push(f);
    }
    fn route(&self, req: Request, res: &mut Response) {
        match req.url {
            "/index" => (self.routes.read().get(0))(req, res),
            "/hello" => (self.routes.read().get(1))(req, res),
            _ => unreachable!(),
        };
    }
    fn copy(&self) -> ~Router:Send {
        ~MatchRouter {
            routes: self.routes.clone(),
        } as ~Router:Send
    }
}

impl App for MyApp {
    fn get_router(&self) -> ~Router:Send {
        let a = ~MatchRouter{routes: Arc::new(RWLock::new(Vec::new()))};
        a.add_route("/index", |req, &mut res| {self.index()});
        a.add_route("/hello", |req, &mut res| {self.hello()});
        a as ~Router:Send
    }
}

impl MyApp {
    fn index(&self) { self.render(); }
    fn hello(&self) { println!("hello!"); }
}


impl<'a> Server for Oxidize<'a> {
    fn get_config(&self) -> http::server::Config {
        let bind_addr = self.conf.bind_addr;
        http::server::Config { bind_address: bind_addr }
    }

    fn handle_request(&self, req: &http::server::Request, response: &mut ResponseWriter) {
        let q = Request{ method: "test" };
        let mut s = Response{ text: ~"hehe" };

        self.router.route(q, &mut s);
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