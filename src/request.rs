
// use std::hashmap::HashMap;
// TODO This is what we want the user to receive NOT a rust-http::request
// The reasoning is we want the public API to be decoupled from any one server implementation
// so that we can have a web framework work on any backend :)
// Until then though we are just reexporting rust-http::Request
// pub struct Request {
//     method : ~str,
//     GET : HashMap<~str, ~str>,
//     POST : HashMap<~str, ~str>,
// }
