extern mod http;

use http::server::Request;

use std::hashmap::HashMap;

pub trait Renderer {
	fn render(&self, request: &Request, file_name: &str, context : Option<HashMap<&str, &str>>) -> ~str;
}