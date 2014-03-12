use http;
use http::method::Method;
use collections::hashmap::HashMap;
use std::default::Default;

#[allow(uppercase_variables)]
pub struct Request {
    method : Method,
    uri: ~str,
    GET : Option<HashMap<~str, ~str>>,
    POST : Option<HashMap<~str, ~str>>,
    context : Option<HashMap<~str,~str>>
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