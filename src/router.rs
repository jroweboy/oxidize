extern mod http;

use http::server::{Request,ResponseWriter};


pub trait Router {
	fn route(&self, request: &Request, response: &mut ResponseWriter) -> ~str;
}