extern crate http;
extern crate mustache;
extern crate serialize;

//use http::server::Request;
use request::Request;

use std::io::File;
use std::str::from_utf8;

#[deriving(Encodable)]
pub struct Context;


#[allow(unused_variable)]
pub fn render(request: Request, file_name: &str, context : Option<Context>) -> ~str {
	// TODO: the templates dir probably shouldn't be hard coded
	let path = Path::new("templates/"+file_name);
	println!("Render for this file: {}", path.display());
    let contents = File::open(&path).read_to_end();

    match from_utf8(contents.unwrap()) {
        Some(str) => str.to_owned(),
        None => fail!("File could not be rendered")
    }
}

#[allow(unused_variable)]
pub fn mustache_render(request: Request, file_name: &str, context : Option<Context>) -> ~str {
	let path = Path::new("templates/"+file_name);
	println!("Render for this file: {}", path.display());
    let contents = File::open(&path).read_to_end();

	let c = match context {
		Some(con) => con,
		None => Context
	};
	// TODO: add the request to the context so that they can use things like session vars
    let conts = match from_utf8(contents.unwrap()) {
        Some(str) => str.to_owned(),
        None => fail!("File could not be parsed as UTF8")
    };
	mustache::render_str(conts, &c)
}