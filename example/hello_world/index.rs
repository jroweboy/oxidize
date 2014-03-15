extern crate oxidize;
extern crate collections;

use oxidize::{Oxidize, Request, ResponseWriter, Config, MediaType};
use oxidize::route::Router;
use oxidize::route::regexrouter::{RegexRouter, RegexRoute, Route, Regex, Simple};
use oxidize::renderer::{render, mustache_render};
use oxidize::status;
use collections::hashmap::HashMap;
use std::io::net::ip::SocketAddr;

#[allow(unused_must_use)]
fn index(request: &Request, response: &mut ResponseWriter) {
    // TODO: maybe make a macro to make this look nicer? But this is faster at least :p
    response.status = status::Ok;
    response.write_content_auto(
        MediaType {type_: ~"text",subtype: ~"html",parameters: ~[]}, 
        render("index.html")
    );
}

#[allow(unused_must_use)]
fn test_mustache(request: &Request, response: &mut ResponseWriter) {
    let mut context = HashMap::<~str, ~str>::new();
    context.insert(~"firstName", ~"Jim");
    context.insert(~"lastName", ~"Bob");
    context.insert(~"blogURL", ~"http://notarealurl.com :p");
    context.insert(~"reverse_index", 
        request.reverse("index", None).unwrap_or(&~"no such route").to_owned());

    response.status = status::Ok;
    response.write_content_auto(
        MediaType {type_: ~"text",subtype: ~"html",parameters: ~[]}, 
        mustache_render("mustache.html", Some(&context))
    );
}

#[allow(unused_must_use)]
fn test_variable(request: &Request, response: &mut ResponseWriter) {
    response.status = status::NotImplemented;
    response.write_content_auto(
        MediaType{type_: ~"text",subtype: ~"html",parameters: ~[]}, 
        render("500.html")
    );
}

fn main() {
    // TODO capture groups currently clash :S I need to rework that a bit
    let routes: ~[RegexRoute] = ~[
        Regex(Route{ method: "GET", name: "index", path: "^/$", fptr: index}),
        Regex(Route{ method: "GET", name: "test_mustache", path: "^/test/?$", fptr: test_mustache}),
        //Regex(Route{ method: "GET", path: "^/test/(?P<year>\\d{4})/?$", fptr: test_variable}),
        Simple(Route{ method: "GET", name: "test_variable", path: "/simple/:var/", fptr: test_variable}),
        //Simple(Route{ method: "GET", path: "/simple/:another/test-:stuff/", fptr: test_mustache}),
    ];

    let router = ~RegexRouter::new(routes);
    let conf = Config {
        debug: true,
        bind_addr: from_str::<SocketAddr>("127.0.0.1:8001").unwrap(),
    };

    let server = Oxidize::new(conf, router as ~Router);
    server.serve();
}