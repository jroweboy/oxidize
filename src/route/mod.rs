use request::Request;
use http::server::ResponseWriter;
use collections::hashmap::HashMap;

pub trait Router {
    // TODO: a router should have a way to call the middleware stack before and after the view
    fn route(&self, request: &mut Request, response: &mut ResponseWriter);
    fn reverse(&self, name: &str, vars: Option<HashMap<~str,~str>>) -> Option<&~str>;
    fn copy(&self) -> ~Router;
}

impl Clone for ~Router {
    fn clone(&self) -> ~Router {
        self.copy()
    }
}

pub mod regexrouter;