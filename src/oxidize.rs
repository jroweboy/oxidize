//! oxidize is the main controller for the application. It is responsible for 
//! handling the http-request and converting it to a framework friendly request.
//! Other responsibilities include managing middleware (TODO), detecting task failure (TODO),
//! routing urls, serving static files (and allow one to serve these files 
//! from something like varnish if they set that up), and configuring the http response
//! This struct can be considered the "brain" of oxidize and it controls all the different 
//! parts which are all self contained (and tested TODO)

pub use app::App;
pub use conf::Config;
pub use http::status;
pub use request::Request;
pub use middleware::MiddleWare;
pub use http::server::ResponseWriter;
// TODO Ogeon wrote a wonderful MediaType macro that I would love to incorporate
pub use http::headers::content_type::MediaType;

use route::Router;
use request;

use http;
use http::server::{Server, ResponseWriter}; 
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};
use sync::{Arc, RWLock};
use time;

/// The Oxidize struct contains a handler to all of the major aspects of the framework
/// As such, the user is free to use whatever router they desire as long as it implements
/// the router Trait. In addition, MiddleWare can be added that will be able to mutate
/// both request and response 
#[deriving(Clone)]
struct Oxidize {
    pub conf : Arc<Config>,
    pub router : Arc<~Router:Send+Share>,
    pub app : Arc<~App:Send+Share>,
    pub filters : Option<Arc<RWLock<Vec<~MiddleWare:Send+Share>>>>,
}

impl Oxidize {
    /// A simple constructor for the framework user so they don't have to worry about
    /// which concurrency primitives I use (aka ignore the implementation details)
    pub fn new(conf: Config, router: ~Router:Send+Share, 
                app: ~App:Send+Share, filters: Option<Vec<~MiddleWare:Send+Share>>) -> Oxidize {
        if filters.is_some() {
            Oxidize {
                conf: Arc::new(conf),
                router: Arc::new(router),
                app: Arc::new(app),
                filters: Some(Arc::new(RWLock::new(filters.unwrap()))),
            }
        } else {

            Oxidize {
                conf: Arc::new(conf),
                router: Arc::new(router),
                app: Arc::new(app),
                filters: None,
            }
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
            Star => ~"error"
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
            router: self.router.clone(),
        };

        response.headers.date = Some(time::now_utc());
        // TODO, store the StrBuf in oxidize so it doesn't get remade each time
        response.headers.server = Some(StrBuf::from_str("Oxidize/0.2.0 (Ubuntu)"));
        // TODO: hide the response writer since we still want middle ware to be able to
        // write to a response if needed and if the framework user calls ResponseWriter.write
        // it will close the stream and cause the middleware to fail.
        // TODO: GET isn't the only kind of method we will use
        let route_info = self.router.route("GET", uri);
        self.app.handle_route(route_info, my_request, response)
    }
}
