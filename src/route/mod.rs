use request::Request;
use http::server::ResponseWriter;
use collections::hashmap::HashMap;

pub trait Router {
    fn route(&self, request: &mut Request, response: &mut ResponseWriter);
    fn reverse<'a>(&'a self, name: &str, vars: Option<HashMap<~str,~str>>) -> Option<&'a ~str>;
    fn copy(&self) -> ~Router;
}

impl Clone for ~Router {
    fn clone(&self) -> ~Router {
        self.copy()
    }
}

pub mod regexrouter;