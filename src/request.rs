// TODO make these a submodule. I heard they are pretty neat :p

extern crate http;
extern crate collections;

use collections::hashmap::HashMap;
use http::method::Method;

pub struct Request {
    method : Method,
    uri: ~str,
    GET : Option<HashMap<~str, ~str>>,
    POST : Option<HashMap<~str, ~str>>,
}

