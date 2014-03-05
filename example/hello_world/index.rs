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



fn index<'a>(request: &'a Request) -> &'a Response {
    Response {
    	status: status::Ok, 
    	content: render(&request, "index.html")
    }
}

// TODO: why do controllers need to return an owned pointer? Should it do that?
fn test_mustache<'a>(request: &'a Request) -> &'a Response {
    //let mut context = request.context.unwrap_or(Hashmap::<~str, ~str>new());

    Response {
        status: status::Ok, 
        // TODO: I don't like having to pass Some(empty_val) to say None
        content: mustache_render(&request, "mustache.html")
    }
}

// TODO: why do controllers need to return an owned pointer? Should it do that?
fn test_variable<'a>(request: &'a Request) -> &'a Response {
    {
        let mut context = request.context.expect("No context found in the fn test_variable");
    }
    // 
    // let year = request.get_context_var_or_fail("year");
    //do something with the year if you want
    // let context = request.get_context();
    // let year = match request.context {
    //     Some(str) => str.get("year".to_owned()),
    //     None => fail!("No year was passed in :p Thats weird"),
    // };

    Response {
        status: status::Ok, 
        content: mustache_render(&request, "variable.html"),
    }
}


fn main() {
    // TODO: rework the params to be more sensible :p See oxidize.rs Oxidize struct for more info
    let server = Oxidize::new(8000, "localhost", routes);
    server.serve();
}