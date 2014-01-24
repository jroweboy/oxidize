
extern mod oxidize;

use oxidize::Request;
use oxidize::Response;
use oxidize::render;
use std::hashmap::HashMap;

pub fn index(Request) -> Response {
	return Response::new(200, render(request, "templates/index.html", None))
}