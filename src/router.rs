extern mod http;

use http::server::ResponseWriter;


pub trait Router {
	fn route(&self, path: &str, response: &mut ResponseWriter) -> ~str;
}