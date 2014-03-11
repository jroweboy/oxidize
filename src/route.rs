extern crate http;

use response::Response;
use request::Request;

// Left off here
// TODO change this to a &mut so that it moves the request.
pub type View = fn (&mut Request) -> Response; //fn<'a>(&'a Request) -> &'a Response;

//#[deriving(Clone)]
pub struct Route<'r> {
	method : &'r str,
	name : &'r str,
	path : &'r str,
	fptr : View, // fn(&'a Request) -> &'a Response
}

impl<'r> Route<'r> {
    pub fn call(&self, request: &mut Request) -> Response {
        println!("Routing calling the function for path [{}]", self.path);
        (self.fptr)(request)
        //call(request)
    }
}
