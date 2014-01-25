//extern mod oxidize;

//extern mod http;

//use oxidize::Oxidize;
//use oxidize::Request;
//use oxidize::Response;
//use oxidize::render;
//use oxidize::Router;
use std::hashmap::HashMap;


/////////////////////////
// These are functions that I want Oxidize to define
// I'm just putting them in here for now
/////////////////////////

//enum Method {
//	GET,
//	POST,
//	WS
//}



pub struct User {
	id : uint,
	is_anonymous : bool,
	is_authenticated : bool,
	is_superuser : bool,
}

pub struct Request {
	method : ~str, //Method,
    GET : HashMap<~str, ~str>,
    POST : HashMap<~str, ~str>,
}

pub struct Response {
	content : ~str,
	status_code : uint,
	// add more stuff? I dunno
	//reason_phrase : ~str,
	//streaming : bool,
}

impl Response {
	fn new(status_code : uint, content : ~str) -> Response {
		 Response { content: content, status_code: status_code } 
	}
}

// todo: Make the context vars better
fn render(request : Request, template : &str, context : Option<HashMap<&str, &str>>) -> ~str {
	// load the template file and run it through the parser
	~"NOTHING HAHAHAHAHA"
}

type Route<'a> = (&'a str, &'a str, fn(Request) -> Response);


//////////////////////////
// This is what I want the example to look like
//////////////////////////

static routes: &'static [Route<'static>] = &[
	("GET", "/", index),
	("GET", "/test", index)
];


pub fn index(request: Request) -> Response {
	return Response::new(200, render(request, "templates/index.html", None));
}

fn main() {
	//let server = Oxidize::new(8000);
	//server.serve_forever();	
}