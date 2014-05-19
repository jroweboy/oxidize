//! oxidize is the main controller for the application. It is responsible for 
//! handling the http-request and converting it to a framework friendly request.
//! Other responsibilities include managing middleware (TODO), detecting task failure (TODO),
//! routing urls, serving static files (and allow one to serve these files 
//! from something like varnish if they set that up), and configuring the http response
//! This struct can be considered the "brain" of oxidize and it controls all the different 
//! parts which are all self contained (and tested TODO)

use app::App;
use conf::Config;
use request::Request;
use middleware::MiddleWare;

use router::Router;
use request;

use http;
use http::server::{Server, ResponseWriter}; 
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};
use sync::{Arc, RWLock};
use time;

/// The Oxidize struct contains a handler to all of the major aspects of the framework
/// MiddleWare can be added that will be able to mutate both request and response 
#[deriving(Clone)]
pub struct Oxidize {
    conf : Arc<Config>,
    router : Arc<Router<&'static str>>,
    app : Arc<Box<App:Send+Share>>,
    // obviously my favorite type in the whole program. maybe worth a typedef...
    filters : Option<Arc<RWLock<Vec<Box<MiddleWare:Send+Share>>>>>,
}

impl Oxidize {
    /// Creates a new Oxidize which contains the main guts of the web application
    pub fn new(conf: Config, app: Box<App:Send+Share>, filters: Option<Vec<Box<MiddleWare:Send+Share>>>) -> Oxidize {
    
        Oxidize {
            conf: Arc::new(conf),
            router: Arc::new(app.get_router()),
            app: Arc::new(app),
            filters: filters.map(|f| {
                Arc::new(RWLock::new(f))
            }),
        }
    }

    /// As silly as it sounds, rust-http does not expose the `serve_forever` method,
    /// so this is a simple workaround to enable the framework user to start the server.
    pub fn serve(self) {
        debug!("Server is now running at {}", self.conf.bind_addr.to_str());
        self.serve_forever();
    }

}

impl Server for Oxidize {
    /// `get_config` is part of the Server trait. rust-http uses this to 
    /// bind to the proper address and thats about it. Maybe when teepee
    /// comes around this will not be needed anymore?
    fn get_config(&self) -> http::server::Config {
        let bind_addr = self.conf.bind_addr;
        http::server::Config { bind_address: bind_addr }
    }

    /// Receives a request from rust-http and cleans it up for use by the 
    /// framework user. It will call the routing functions and any middleware
    /// and pass the app context and a request and response to the user.
    fn handle_request(&self, req: &http::server::Request, response: &mut ResponseWriter) {
        // create a request object
        let path = match req.request_uri {
            AbsolutePath(ref i) => i.to_str(),
            AbsoluteUri(ref i) => i.to_str(),
            Authority(ref i) => i.to_str(),
            Star => "error".to_owned()
        };
        // TODO support any kind of method
        let test_method = match from_str("GET") {
            Some(m) => m,
            None => http::method::Get
        };

        let uri = path.clone();
        // TODO add any GET POST vars here as well
        // TODO standardize any usages of uri/url (first pick one and stick with it)
        let my_request = &mut request::Request {
            method: test_method, 
            uri: path,
            GET: None,
            POST: None,
            user: None,
            // router: self.router.clone(),
        };

        response.headers.date = Some(time::now_utc());
        // TODO, store the StrBuf in oxidize so it doesn't get remade each time
        response.headers.server = Some(StrBuf::from_str("Oxidize/0.2.0 (Ubuntu)"));
        // TODO: hide the response writer since we still want middle ware to be able to
        // write to a response if needed and if the framework user calls ResponseWriter.write
        // it will close the stream and cause the middleware to fail.
        // TODO: GET isn't the only kind of method we will use
        // TODO: Do we want to just hand the RouterResult straight to the app?
        let route_info = self.router.find(http::method::Get, uri);
        if route_info.is_some() {
            let (name, vars) = route_info.unwrap();
            self.app.handle_route(Some(name), Some(vars), my_request, response);
        } else {
            // 404
            self.app.handle_route(None, None, my_request, response);
        }
    }
}
