extern crate oxidize;
extern crate collections;
extern crate serialize;

use oxidize::{Oxidize, Request, ResponseWriter, Config, MediaType};
use oxidize::route::Router;
use oxidize::route::regexrouter::{RegexRouter, RegexRoute, Route, Regex, Simple};
use oxidize::renderer::{render, mustache_render};
use oxidize::status;
use collections::hashmap::HashMap;
use std::io::net::ip::SocketAddr;
use std::io;
use serialize::{json, Encodable};

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
    let routes: ~[RegexRoute] = ~[
        Regex(Route{ method: "GET", name: "index", path: "^/$", fptr: index}),
        Simple(Route{ method: "GET", name: "json", path: "/json", fptr: json_handler}),
        Simple(Route{ method: "GET", name: "plaintext", path: "/plaintext", fptr: plaintext_handler}),
    ];

    let router = ~RegexRouter::new(routes);
    let conf = Config {
        debug: true,
        bind_addr: from_str::<SocketAddr>("127.0.0.1:8001").unwrap(),
    };

    let server = Oxidize::new(conf, router as ~Router);
    server.serve();
}


 #[deriving(Encodable)]
 pub struct JSON_Message   {
    message: ~str,
 }

#[allow(unused_must_use)]
fn json_handler(request: &Request, response: &mut ResponseWriter) {
    let message = JSON_Message{message:~"Hello, World!"};
    response.write_content_auto(
        MediaType {type_: ~"application",subtype: ~"javascript",parameters: ~[]}, 
        json::Encoder::str_encode(&message)
    );
}

#[allow(unused_must_use)]
fn plaintext_handler(request: &Request, response: &mut ResponseWriter) {
    response.write_content_auto(
        MediaType {type_: ~"text",subtype: ~"plain",parameters: ~[]}, 
        ~"Hello, World!"
    );
}