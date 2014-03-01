extern crate http;

use response::Response;
use request::Request;

pub type View = fn(Request) -> ~Response;

#[deriving(Clone)]
pub struct Route<'r> {
	method : &'r str,
	path : &'r str,
	fptr : View
}

impl<'r> Route<'r> {
    pub fn call (&self, request: Request) -> ~Response {
        println!("Routing calling the function for path [{}]", self.path);
        let call = self.fptr;
        call(request)
    }
}
