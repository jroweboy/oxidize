extern crate url;

use http::method::Method;
use collections::hashmap::HashMap;
use std::cell::Ref;
use route::Router;

#[allow(uppercase_variables)]
pub struct Request<'a> {
    pub method : Method,
    pub uri: ~url::Url,
    pub GET : Option<HashMap<~str, ~str>>,
    pub POST : Option<HashMap<~str, ~str>>,
    pub context : Option<HashMap<~str,~str>>,
    pub router : Ref<'a, ~Router:Send>,
}

impl<'a> Request<'a> {
    pub fn reverse<'a>(&'a self, name: &str, vars: Option<~[(~str,~str)]>) -> Option<~str> {
        self.router.reverse(name, vars)
    }
}