extern crate http;

use response::Response;
use request::Request;

pub type View = fn (&Request) -> Response; //fn<'a>(&'a Request) -> &'a Response;

//#[deriving(Clone)]
pub struct Route<'r> {
	method : &'r str,
	path : &'r str,
	fptr : View, // fn(&'a Request) -> &'a Response
}

impl<'r> Route<'r> {
    pub fn call(&self, request: &Request) -> Response {
        println!("Routing calling the function for path [{}]", self.path);
        (self.fptr)(request)
        //call(request)
    }
}
