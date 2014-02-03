extern mod http;

use response::Response;
// TODO remove rust-http from the public facing api
//use request::Request;
use http::server::request::Request;

pub struct Route<'r> {
	method : &'r str,
	path : &'r str,
	fptr : fn(&Request) -> ~Response
}

impl<'r> Route<'r> {
    pub fn call (&self, request : &Request) -> ~Response {
        println!("Routing calling the function for path [{}]", self.path);
        let call = self.fptr;
        call(request)
    }
}