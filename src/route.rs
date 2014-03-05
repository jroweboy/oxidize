extern crate http;

use response::Response;
use request::Request;

pub type View<'a> = fn<'a>(&'a Request) -> &'a Response;

//#[deriving(Clone)]
pub struct Route<'r, 'a> {
	method : &'r str,
	path : &'r str,
	fptr : fn(&'a Request) -> &'a Response
}

impl<'r, 'a> Route<'r, 'a> {
    pub fn call<'a>(&'a self, request: &'a Request) -> &'a Response {
        println!("Routing calling the function for path [{}]", self.path);
        let call = self.fptr;
        call(request)
    }
}
