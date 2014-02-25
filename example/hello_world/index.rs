extern crate oxidize;

use oxidize::{Oxidize, Request, Response, Route};
use oxidize::renderer::{render, mustache_render};
// reexported the status from rust-http ahahaha
use oxidize::status;


// TODO maybe make an awesome macro to allow a user to declare a beautiful looking routes
static routes: &'static [Route<'static>] = &[
    Route { method: "GET", path: "^/$", fptr: index},
    Route { method: "GET", path: "^/test/?$", fptr: test_mustache},
    Route { method: "GET", path: "^/test/(?P<year>\\d{4})/?$", fptr: test_variable},
];



fn index(request: Request) -> ~Response {
    ~Response {
    	status: status::Ok, 
    	content: render(request, "index.html", None)
    }
}

// TODO: why do controllers need to return an owned pointer? Should it do that?
fn test_mustache(request: Request) -> ~Response {
    ~Response {
        status: status::Ok, 
        content: mustache_render(request, "mustache.html", None)
    }
}

// TODO: why do controllers need to return an owned pointer? Should it do that?
fn test_variable(request: Request) -> ~Response {
    ~Response {
        status: status::Ok, 
        content: mustache_render(request, "mustache.html", None)
    }
}


fn main() {
    // TODO: rework the params to be more sensible :p See oxidize.rs Oxidize struct for more info
    let server = Oxidize::new(8000, "localhost", routes);
    server.serve();
}