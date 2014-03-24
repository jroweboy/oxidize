extern crate oxidize;
extern crate collections;

use oxidize::{Oxidize, Request, ResponseWriter, Config, MediaType};
use oxidize::route::Router;
use oxidize::route::trierouter::{TrieRouter, TrieRoute, Route, Simple, Variable};
use oxidize::renderer::{render, mustache_render};
use oxidize::status;
use collections::hashmap::HashMap;
use std::io::net::ip::SocketAddr;

#[allow(unused_must_use)]
fn index(request: &Request, response: &mut ResponseWriter, vars: &~[(~str,~str)]) {
    // TODO: maybe make a macro to make this look nicer? But this is faster at least :p
    response.status = status::Ok;
    response.write_content_auto(
        MediaType {type_: ~"text",subtype: ~"html",parameters: ~[]}, 
        render("index.html")
    );
}

#[allow(unused_must_use)]
fn test_mustache(request: &Request, response: &mut ResponseWriter, vars: &~[(~str,~str)]) {
    let mut context = HashMap::<~str, ~str>::new();
    context.insert(~"firstName", ~"Jim");
    context.insert(~"lastName", ~"Bob");
    context.insert(~"blogURL", ~"http://notarealurl.com :p");
    context.insert(~"reverse_index", 
        request.reverse("index", None).unwrap_or(~"no such route").to_owned());

    response.status = status::Ok;
    response.write_content_auto(
        MediaType {type_: ~"text",subtype: ~"html",parameters: ~[]}, 
        mustache_render("mustache.html", Some(&context))
    );
}

#[allow(unused_must_use)]
fn test_variable(request: &Request, response: &mut ResponseWriter, vars: &~[(~str,~str)]) {
    let mut context = HashMap::<~str, ~str>::new();

    // the clone is required for some reason...
    for var in vars.iter() {
        let t = var.clone();
        let (k,v) = t;
        context.insert(k,v);
    }
    context.insert(~"reverse",
        request.reverse("test_variable", Some(vars.to_owned())).unwrap_or(~"no such route").to_owned());

    response.status = status::Ok;
    response.write_content_auto(
        MediaType{type_: ~"text",subtype: ~"html",parameters: ~[]}, 
        mustache_render("variable.html", Some(&context))
    );
}

fn main() {
    let routes: ~[TrieRoute] = ~[
        Simple(Route{ method: "GET", name: "index", path: "/", fptr: index}),
        Simple(Route{ method: "GET", name: "test_mustache", path: "/test", fptr: test_mustache}),
        Variable(Route{ method: "GET", name: "test_variable", path: "/users/user-:userid/post-:postid", fptr: test_variable}),
    ];

    let router = ~TrieRouter::new(routes);
    let conf = Config {
        debug: true,
        bind_addr: from_str::<SocketAddr>("127.0.0.1:8001").unwrap(),
    };

    let server = Oxidize::new(conf, router as ~Router);
    server.serve();
}