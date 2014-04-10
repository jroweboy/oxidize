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
    pub conf : &'a Config,
    // pub app : &'a App,
    pub router : RefCell<~Router<'a>:Send>,
    // pub filters : ~[~MiddleWare],
}

trait App<'a> {
    fn get_router(&self) -> ~Router<'a>:Send;
}

struct MyApp {
    renderer: Renderer,
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

trait Router<'a>:Send {
    fn add_route(&self, &str, |Request, &mut Response|: 'a);
    fn route(&self, Request, &mut Response);
    fn copy(&self) -> ~Router<'a>:Send;
}

impl<'a> Clone for ~Router<'a>:Send {
    fn clone(&self) -> ~Router<'a>:Send {
        self.copy()
    }
}

// not fully operational but good enough for now
struct MatchRouter<'a> {
    // make this a tuple of uris method and closure?
    routes : Arc<RWLock<~[|Request, &mut Response|: 'a]>>,
}

impl<'a> Router<'a> for MatchRouter<'a> {
    fn add_route(&self, path: &str, f: |Request, &mut Response|: 'a){
        // self.routes.write().push(f);
    }
    fn route(&self, req: Request, res: &mut Response) {
        match req.url {
            "/index" => (*self.routes.read().get(0).unwrap())(req, res),
            "/hello" => (*self.routes.read().get(1).unwrap())(req, res),
            _ => unreachable!(),
        };
    }
    fn copy(&self) -> ~Router<'a>:Send {
        ~MatchRouter {
            routes: self.routes.clone(),
        } as ~Router<'a>:Send
    }
}

impl<'a> App<'a> for MyApp {
    fn get_router(&self) -> ~Router<'a>:Send {
        let routes : ~[|Request, &mut Response|: 'a] = ~[
            |req, res| {self.index()},
            |req, res| {self.hello()},
        ];
        let a = ~MatchRouter{routes: Arc::new(RWLock::new(routes))};
        // a.add_route("/index", |req, res| {self.index()});
        // a.add_route("/hello", |req, res| {self.hello()});
        a as ~Router<'a>:Send
    }
}

impl MyApp {
    fn index(&self) { self.renderer.print(); }
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