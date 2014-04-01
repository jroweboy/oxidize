use http::method::Method;
use collections::hashmap::HashMap;
use std::cell::Ref;
use route::Router;

#[allow(uppercase_variables)]
pub struct Request<'a> {
    method : Method,
    uri: ~str,
    GET : Option<HashMap<~str, ~str>>,
    POST : Option<HashMap<~str, ~str>>,
    context : Option<HashMap<~str,~str>>,
    router : Ref<'a, ~Router>,
}

impl<'a> Request<'a> {
    pub fn reverse<'a>(&'a self, name: &str, vars: Option<~[(~str,~str)]>) -> Option<~str> {
        self.router.reverse(name, vars)
    }
}