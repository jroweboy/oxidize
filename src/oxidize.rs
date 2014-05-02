//! oxidize is the main controller for the application. It is responsible for 
//! handling the http-request and converting it to a framework friendly request.
//! Other responsibilities include managing middleware (TODO), detecting task failure (TODO),
//! routing urls, serving static files (and allow one to serve these files 
//! from something like varnish if they set that up), and configuring the http response
//! This struct can be considered the "brain" of oxidize and it controls all the different 
//! parts which are all self contained (and tested TODO)

// // the glorious logging crate
// #![feature(phase)]
// #[phase(syntax, link)] extern crate log;
// // holds references to HashMap
// extern crate collections;
// // need to pass the time info to 
// extern crate time;
// // needed for some Encodable stuff
// extern crate serialize;
// // templating is provided by rust-mustache -- making that required in userland only!
// // extern crate mustache;
// // handles all the http stuff
// extern crate http;
// // used for holding the pcre struct in a mutable multithreaded way
// extern crate sync;

// It turns out its real easy to reexport mods :D
// People that extern mod oxidize can use oxidize::reexported::mod;
// pub use http::status;
// pub use request::Request;
// pub use conf::Config;
// pub use http::server::ResponseWriter;
// pub use http::headers::content_type::MediaType;

use route::Router;
// use sync::Arc;
// use std::cell::RefCell;

pub use app::App;
pub use conf::Config;
pub use http::status;
pub use request::Request;
pub use http::server::ResponseWriter;
// TODO Ogeon wrote a wonderful MediaType macro that I would love to incorporate
pub use http::headers::content_type::MediaType;

use sync::{Arc, RWLock};
use http::server::{Server, ResponseWriter}; 
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};

//! The Oxidize struct contains a handler to all of the major aspects of the framework
//! As such, the user is free to use whatever router they desire as long as it implements
//! the router Trait. In addition, MiddleWare can be added that will be able to mutate
//! both request and response 
#[deriving(Clone)]
struct Oxidize {
    pub conf : Arc<Config>,
    pub router : Arc<~Router:Send+Share>,
    pub app : Arc<~App:Send+Share>,
    pub filters : Arc<RWLock<Vec<~MiddleWare:Send+Share>>>,
}

impl Oxidize {
    //! A simple constructor for the framework user so they don't have to worry about
    //! which concurrency primitives I use (aka ignore the implementation details)
    pub fn new(conf: Config, router: ~Router:Send+Share, 
                app: ~App:Send+Share, filters: ~MiddleWare:Send+Share) -> Oxidize {
        Oxidize {
            conf: Arc::new(conf),
            router: Arc::new(router),
            app: Arc::new(app),
            filters: Arc::new(RWLock::new(Vec::new(filters))),
        }
    }

    //! As silly as it sounds, rust-http does not expose the `serve_forever` method,
    //! so this is a simple workaround to enable the framework user to start the server.
    pub fn serve(self) {
        debug!("Server is now running at {}", self.conf.bind_addr.to_str());
        self.serve_forever();
    }

}

impl Server for Oxidize {
    //! `get_config` is part of the Server trait. rust-http uses this to 
    //! bind to the proper address and thats about it. Maybe when teepee
    //! comes around this will not be needed anymore?
    fn get_config(&self) -> http::server::Config {
        let bind_addr = self.conf.bind_addr;
        http::server::Config { bind_address: bind_addr }
    }

    //! Receives a request from rust-http and cleans it up for use by the 
    //! framework user. It will call the routing functions and any middleware
    //! and pass the app context and a request and response to the user.
    fn handle_request(&self, req: &http::server::Request, response: &mut ResponseWriter) {
        // create a request object
        let path = match req.request_uri {
            AbsolutePath(ref i) => i.to_str(),
            AbsoluteUri(ref i) => i.to_str(),
            Authority(ref i) => i.to_str(),
            Star => ~"error"
        };
        // TODO support any kind of method
        let test_method = match from_str("GET") {
            Some(m) => m,
            None => http::method::Get
        };
        // TODO add any GET POST vars here as well
        // TODO standardize any usages of uri/url (first pick one and stick with it)
        let my_request = &mut request::Request {
            method: test_method, 
            uri: path,
            GET: None,
            POST: None,
            user: None,
            router: self.router.clone(),
        };

        response.headers.date = Some(time::now_utc());
        response.headers.server = Some(~"Oxidize/0.2.0 (Ubuntu)");
        // TODO: hide the response writer since we still want middle ware to be able to
        // write to a response if needed and if the framework user calls ResponseWriter.write
        // it will close the stream and cause the middleware to fail.
        self.app.handle_route(self.router.route(path), &my_request, response)
    }
}
