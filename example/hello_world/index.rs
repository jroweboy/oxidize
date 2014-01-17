
extern mod Oxidize;

use Oxidize::Request;
use Oxidize::Response;
use Oxidize::render;
use std::hashmap::HashMap;

pub fn index(Request) -> Response {
	return Response::new(200, render(request, "templates/index.html", None))
}