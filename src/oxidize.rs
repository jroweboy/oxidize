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

use http;
use http::server::{Server, ResponseWriter};
use sync::{Arc, RWLock};
use time;


static version : &'static str = "Oxidize/0.2.0 (Ubuntu)";

/// The Oxidize struct contains a handler to all of the major aspects of the framework
/// MiddleWare can be added that will be able to mutate both request and response 
#[deriving(Clone)]
pub struct Oxidize {
    conf : Arc<Config>,
    router : Arc<Router<&'static str>>,
    app : Arc<Box<App:Send+Share>>,
    // obviously my favorite type in the whole program. maybe worth a typedef...
    filters : Option<Arc<RWLock<Vec<Box<MiddleWare:Send+Share>>>>>,
    server_version : Arc<Option<StrBuf>>,
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
            server_version : Arc::new(Some(StrBuf::from_str(version))),
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
        let mut my_request = Request::new(req);

        response.headers.date = Some(time::now_utc());
        // TODO: find a way to remove this clone?
        response.headers.server = self.server_version.deref().clone();
        // TODO: hide the response writer since we still want middle ware to be able to
        // write to a response if needed and if the framework user calls ResponseWriter.write
        // it will close the stream and cause the middleware to fail.
        // TODO: GET isn't the only kind of method we will use
        // TODO: Do we want to just hand the RouterResult straight to the app?
        let route_info = self.router.find(my_request.method.clone(), my_request.uri.to_str());
        if route_info.is_some() {
            let (name, vars) = route_info.unwrap();
            self.app.handle_route(Some(name), Some(vars), &mut my_request, response);
        } else {
            // 404
            self.app.handle_route(None, None, &mut my_request, response);
        }
    }
}

#[cfg(test)]
mod test {
    use collections::hashmap::HashMap;
    #[allow(unused_imports)]
    use http::method::{Get, Post, Delete, Put, Head};
    use super::Oxidize;
    use std::io::net::ip::SocketAddr;

    struct TestApp;
    impl ::app::App for TestApp {
        #[allow(unused_variable)]
        fn handle_route<'a>(&self, route: Option<&&'static str>, vars: Option<HashMap<~str,~str>>,
                         req: &mut ::request::Request, res: &mut ::http::server::ResponseWriter) {
            // Do nothing. Lets just say it doesn't fail for testing purposes right now.
            // Eventually I'll want to make a separate App that fails on handle route 
            // to test how oxidize handles task failure.
        }

        fn get_router(&self) -> ::router::Router<&'static str> {
            // TODO once oxidize supports things other than GET requests, I'll want to add them in as test cases here
            let routes = [
                (Get, "/", "index"),
            ];
            ::router::Router::from_routes(routes)
        }
    }

    fn test_setup() -> Oxidize {
        let app = box TestApp;
        let conf = ::conf::Config {
            debug: true,
            bind_addr: from_str::<SocketAddr>("127.0.0.1:8001").unwrap(),
        };
        Oxidize::new(conf, app as Box<::app::App:Send+Share>, None)
    }

    #[test]
    fn handle_request() {
        // This is frustrating! It won't let me call the Server methods on Oxidize... 
        // I'll have to figure out a better way to internal test this code
        let o = test_setup();
        assert!(o.get_config().debug == true);
    }
}