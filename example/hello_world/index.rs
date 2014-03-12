extern crate oxidize;
extern crate collections;
extern crate sync;

use oxidize::{Oxidize, Request, ResponseWriter, Config, MediaType};
use oxidize::route::{RegexRouter, RegexRoute, Router};
use oxidize::renderer::{render, mustache_render};
use oxidize::status;

use collections::hashmap::HashMap;
use std::io::net::ip::SocketAddr;
use sync::Arc;


// TODO maybe make an awesome macro to allow a user to declare a beautiful looking routes
static routes: &'static [RegexRoute] = &[
    RegexRoute { method: "GET", path: "^/$", fptr: index},
    RegexRoute { method: "GET", path: "^/test/?$", fptr: test_mustache},
    RegexRoute { method: "GET", path: "^/test/(?P<year>\\d{4})/?$", fptr: test_variable},
];

//SimpleRoute { method: "GET", path: "/test/:year/:month", fptr: test_variable },
//StaticServe { method: "GET", path: "/static/*", directory: "/path/to/files" },

fn index(request: &Request, response: &mut ResponseWriter) {
    // TODO: maybe make a macro to make this look nicer? But is is faster at least :p
    response.status = status::Ok;
    response.write_content_auto(
        MediaType {type_: ~"text",subtype: ~"html",parameters: ~[]}, 
        render("index.html")
    );
}

fn test_mustache(request: &Request, response: &mut ResponseWriter) {
    let mut context = HashMap::<~str, ~str>::new();
    context.insert(~"firstName", ~"Jim");
    context.insert(~"lastName", ~"Bob");
    context.insert(~"blogURL", ~"http://notarealurl.com :p");

    response.status = status::Ok;
    response.write_content_auto(
        MediaType {type_: ~"text",subtype: ~"html",parameters: ~[]}, 
        mustache_render("mustache.html", Some(&context))
    );
}

fn test_variable(request: &Request, response: &mut ResponseWriter) {
    response.status = status::NotImplemented;
    response.write_content_auto(
        MediaType{type_: ~"text",subtype: ~"html",parameters: ~[]}, 
        render("500.html")
    );
}


fn main() {
    // TODO clean this up so its nicer
    let router = ~RegexRouter::new(routes);
    // TODO add defaults to Config
    let conf = Config {
        debug: true,
        bind_addr: from_str::<SocketAddr>("127.0.0.1:8001").unwrap(),
    };

    let server = Oxidize::new(conf, router as ~Router);
    server.serve();
}