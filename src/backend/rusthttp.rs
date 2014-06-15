//! RustHttp struct contains a handler to all of the major aspects of the framework
//! MiddleWare can be added that will be able to mutate both request and response 
extern crate http;

use oxidize::Oxidize;
use backend::OxidizeBackend;
use request;
use response::Response;
use self::http::server::request::{AbsolutePath, AbsoluteUri, Authority, Star};
use self::http::server::{Request, Server, ResponseWriter, Config};

use std::io::net::ip::SocketAddr;
use std::collections::HashMap;
use std::mem::transmute;
// use url::Url;
use url::path_from_str;
use sync::Arc;

#[deriving(Clone)]
/// A backend that uses Rust http to process the request and converts all the request data into the Oxidize structs
pub struct RustHttpBackend {
    host : Arc<Config>,
    oxidize : Oxidize,
}

impl RustHttpBackend {
    /// Creates a new Oxidize which contains the main guts of the web application
    pub fn new(host: &str, oxidize: Oxidize) -> RustHttpBackend {
        RustHttpBackend {
            host: Arc::new(Config { 
                bind_address: from_str::<SocketAddr>(host).expect("Could not parse the host:port information")
            }),
            oxidize: oxidize,
        }
    }

    fn http_request_to_oxidize(req: &Request) -> request::Request {
        debug!("Request URI: {}", req.request_uri);
        let path = match req.request_uri {
            AbsolutePath(ref i) => path_from_str(i.as_slice()).unwrap(),
            AbsoluteUri(ref i) => path_from_str(i.path.as_slice()).unwrap(),
            Authority(ref i) => path_from_str(i.as_slice()).unwrap(),
            Star => fail!("Star option is not supported yet")
        };
        // Add get params to the request
        let mut option_get = None;
        if path.query.len() > 0 {
            let mut get = HashMap::new();
            for q in path.query.iter() {
                let (ref a, ref b) = *q;
                get.insert(a.clone(), b.clone());
            }
            option_get = Some(get);
        }

        // TODO add post params
        request::Request {
            method: unsafe {transmute(req.method.clone())},
            uri: path.to_str().to_string(),
            GET: option_get,
            POST: None,
            user: None,
            cookies: HashMap::new()
        }
    }

    fn write_response_to_http(res: Response, writer: &mut ResponseWriter) {
        writer.status = unsafe{transmute(res.status.clone())};
        let _ = writer.write_content_auto(
            unsafe{transmute(res.content_type.clone())},
            res.content
        );
    }
}

impl OxidizeBackend for RustHttpBackend {
    /// As silly as it sounds, rust-http does not expose the `serve_forever` method,
    /// so this is a simple workaround to enable the framework user to start the server.
    fn serve(self) {
        // debug!("Server is now running at {}", self.conf.bind_addr);
        self.serve_forever();
    }
}

impl Server for RustHttpBackend {
    /// `get_config` is part of the Server trait. rust-http uses this to 
    /// bind to the proper address and thats about it. Maybe when teepee
    /// comes around this will not be needed anymore?
    fn get_config(&self) -> Config {
        *self.host.deref().clone()
    } 

    /// Receives a request from rust-http and cleans it up for use by the 
    /// framework user. It will call the routing functions and any middleware
    /// and pass the app context and a request and response to the user.
    fn handle_request(&self, req: &self::http::server::Request, response: &mut ResponseWriter) {
        // create a request object
        let mut my_request = RustHttpBackend::http_request_to_oxidize(req);
        RustHttpBackend::write_response_to_http(self.oxidize.handle_request(&mut my_request), response);
    }
}