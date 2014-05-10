use http::method::Method;
use collections::hashmap::HashMap;
use route::Router;
use sync::Arc;

#[allow(uppercase_variables)]
pub struct Request<'a> {
    pub method : Method,
    pub uri: ~str,
    pub GET : Option<HashMap<~str, ~str>>,
    pub POST : Option<HashMap<~str, ~str>>,
    pub user : Option<~str>,
    // pub context : Option<HashMap<~str,~str>>,
    pub router : Arc<Box<Router:Send+Share>>,
}

impl<'a> Request<'a> {
    pub fn reverse<'a>(&'a self, name: &str, vars: Option<Vec<(~str,~str)>>) -> Option<~str> {
        self.router.reverse(name, vars)
    }
}