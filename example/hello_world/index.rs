extern crate oxidize;
extern crate collections;

use oxidize::{Oxidize, Request, Response, Route};
use oxidize::renderer::{render, mustache_render};
use oxidize::status;

use collections::hashmap::HashMap;


// TODO maybe make an awesome macro to allow a user to declare a beautiful looking routes
static routes: &'static [Route<'static>] = &[
    Route { method: "GET", path: "^/$", fptr: index},
    Route { method: "GET", path: "^/test/?$", fptr: test_mustache},
    Route { method: "GET", path: "^/test/(?P<year>\\d{4})/?$", fptr: test_variable},
];

//SimpleRoute { method: "GET", path: "/test/:year/:month", fptr: test_variable },
//StaticServe { method: "GET", path: "/static/*", directory: "/path/to/files" },

// TODO write directly into the response instead of making a new one
fn index(request: &mut Request) -> Response {
    Response {
    	status: status::Ok, 
    	content: render(request, "index.html", None)
    }
}

fn test_mustache(request: &mut Request) -> Response {
    //let ref c = request.context;
    let mut context = HashMap::<~str, ~str>::new();//c.unwrap_or(HashMap::<~str, ~str>::new());
    context.insert(~"firstName", ~"Jim");
    context.insert(~"lastName", ~"Bob");
    context.insert(~"blogURL", ~"http://notarealurl.com :p");
    //request.context = Some(context);

    Response {
        status: status::Ok, 
        content: mustache_render(request, "mustache.html", Some(&context))
    }
}

// TODO Performance: What if we pool request and response objects?
fn test_variable(request: &mut Request) -> Response {
    // {
    //     let mut context = request.context.expect("No context found in the fn test_variable");
    // }
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
        content: mustache_render(request, "variable.html", None),
    }
}


fn main() {
    // TODO: rework the params to be more sensible :p 
    // See oxidize.rs Oxidize struct for more info
    let server = Oxidize::new(8000, "localhost", routes);
    server.serve();
}