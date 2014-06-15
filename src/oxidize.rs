//! oxidize is the main controller for the application. It is responsible for 
//! handling the http-request and converting it to a framework friendly request.
//! Other responsibilities include managing middleware (TODO), detecting task failure (TODO),
//! routing urls, serving static files (and allow one to serve these files 
//! from something like varnish if they set that up), and configuring the http response
//! This struct can be considered the "brain" of oxidize and it controls all the different 
//! parts which are all self contained (and tested TODO)

use app::App;
use response::Response;
use conf::Config;
use request::Request;
use middleware::MiddleWare;
use router::Router;

use sync::{Arc, RWLock};
// use std::path::posix::Path;

static version : &'static str = "Oxidize/0.2.0 (Ubuntu)";

/// Oxidize is a struct that will handle requests 
#[deriving(Clone)]
pub struct Oxidize {
    app : Arc<Box<App:Send+Share>>,
    conf: Arc<Config>,
    router : Arc<Router<&'static str>>,
    // obviously my favorite type in the whole program. maybe worth a typedef...
    filters : Option<Arc<RWLock<Vec<Box<MiddleWare:Send+Share>>>>>,
    server_version : Arc<Option<String>>,
}

impl Oxidize {
    /// Creates a new Oxidize which contains the main guts of the web application
    pub fn new(conf: Config, app: Box<App:Send+Share>, filters: Option<Vec<Box<MiddleWare:Send+Share>>>) -> Oxidize {
        Oxidize {
            router: Arc::new(app.get_router()),
            app: Arc::new(app),
            filters: filters.map(|f| {
                Arc::new(RWLock::new(f))
            }),
            server_version : Arc::new(Some(String::from_str(version))),
            conf: Arc::new(conf),
        }
    }

    /// Anything that implements the OxidizeBackend trait will need to call function in order to 
    /// route the request and call the user function. A user of the framework need not call this 
    /// as it is called by the backend.
    pub fn handle_request(&self, req: &mut Request) -> Response {
        // TODO: hide the response writer since we still want middle ware to be able to
        // write to a response if needed and if the framework user calls ResponseWriter.write
        // it will close the stream and cause the middleware to fail.
        // TODO: GET isn't the only kind of method we will use
        // TODO: Do we want to just hand the RouterResult straight to the app?
        let route_info = self.router.find(req.method.clone(), req.uri.as_slice());
        if route_info.is_some() {
            let (name, vars) = route_info.unwrap();
            self.app.handle_route(Some(name), Some(vars), req)
        } else {
            // 404
            self.app.handle_route(None, None, req)
        }
    }
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use http::server::Server;
    #[allow(unused_imports)]
    use http::method::{Get, Post, Delete, Put, Head};
    use super::Oxidize;
    use std::io::net::ip::SocketAddr;

    struct TestApp;
    impl ::app::App for TestApp {
        #[allow(unused_variable)]
        fn handle_route<'a>(&self, route: Option<&&'static str>, vars: Option<HashMap<String,String>>,
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
            // bind_addr: from_str::<SocketAddr>("127.0.0.1:8001").unwrap(),
        };
        Oxidize::new(conf, app as Box<::app::App:Send+Share>, None)
    }

    #[test]
    fn test_check_config() {
        let o = test_setup();
        assert!(o.is_debug() == true);
        assert!(o.get_config().bind_address == from_str::<SocketAddr>("127.0.0.1:8001").unwrap());
    }

    #[test]
    fn test_handle_request() {
        // TODO: find out how to forge a request for testing purposes. My initial attempt is below
        // let req = ::http::server::Request{
        //     remote_addr: Some(from_str::<SocketAddr>("127.0.0.2:8001").unwrap()),
        //     headers: Box<::http::headers::request::HeaderCollection>,
        //     body: "Test Body".to_strbuf(),
        //     method: Get,
        //     request_uri: RequestUri::from_strbuf("localhost:8001/test".to_strbuf()),
        //     close_connection: true,
        //     version: (1, 1)
        // }
    }
}