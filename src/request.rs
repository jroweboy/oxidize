// TODO make these a submodule. I heard they are pretty neat :p


use mustache::encoder::{Encoder, Data, Str, Vec, Map};
use collections::hashmap::HashMap;
use http::method::Method;
use http;

// mustache expects something different :(
// pub struct Request<'a> {
//     method : Method,
//     uri: &'a str,
//     GET : Option<HashMap<&'a str, &'a str>>,
//     POST : Option<HashMap<&'a str, &'a str>>,
//     context : Option<HashMap<&'a str, &'a Data>>,
// }

pub struct Request {
    method : Method,
    uri: ~str,
    GET : Option<HashMap<~str, ~str>>,
    POST : Option<HashMap<~str, ~str>>,
    context : Option<HashMap<~str, Data>>,
}

impl Default for Request {
    fn default () -> Request {
        Request {
            method : http::method::Get,
            uri: "index.html".to_owned(),
            GET: None,
            POST: None,
            context: None
        }
    }
}

impl Request {
    ///
    // A verbose name I know, but my idea is to make an easy way to either
    // get a context variable or 404
    // Right now my "404" is just failing.
    // TODO: better name
    // TODO: easy 404 function
    pub fn get_context_var_or_fail<'a>(&'a self, name: &str) -> &Data {
        match self.context.as_ref().and_then(|map| map.find_equiv(&name)) {
            Some(d) => d,
            None => fail!("No context found"),
        }
    }

    pub fn get_context(&self) -> Data {//&Map<~Str,~Data> {
        match self.context {
            Some(m) => Map(m),
            None => fail!("No context found"),
        }
    }
}