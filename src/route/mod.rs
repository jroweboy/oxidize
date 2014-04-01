use request::Request;
use http::server::ResponseWriter;

pub trait Router {
    fn route(&self, request: &mut Request, response: &mut ResponseWriter);
    fn reverse<'a>(&'a self, name: &str, vars: Option<~[(~str,~str)]>) -> Option<~str>;
    fn copy(&self) -> ~Router;
}

impl Clone for ~Router {
    fn clone(&self) -> ~Router {
        self.copy()
    }
}

//pub mod regexrouter;
pub mod trierouter;