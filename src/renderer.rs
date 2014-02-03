extern mod http;
extern mod mustache;

use http::server::Request;

use std::io::File;
use std::str::from_utf8;

#[deriving(Encodable)]
pub struct Context;

pub fn render(request: &Request, file_name: &str, context : Option<Context>) -> ~str {
	// TODO: the templates dir probably shouldn't be hard coded
	let path = Path::new("templates/"+file_name);
	println!("Render for this file: {}", path.display());
    let contents = File::open(&path).read_to_end();

    from_utf8(contents).to_owned()
}

pub fn mustache_render(request: &Request, file_name: &str, context : Option<Context>) -> ~str {
	let path = Path::new("templates/"+file_name);
	println!("Render for this file: {}", path.display());
    let contents = File::open(&path).read_to_end();

	let c = match context {
		Some(con) => con,
		None => Context
	};
	// TODO: add the request to the context so that they can use things like session vars
	mustache::render_str(from_utf8(contents).to_owned(), &c)
}