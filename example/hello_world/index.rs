extern crate oxidize;
extern crate collections;

use oxidize::{Oxidize, Request, ResponseWriter, Config, MediaType};
use oxidize::route::Router;
use oxidize::route::trierouter::{TrieRouter, TrieRoute, Route, StaticRoute, Variable, Static};
use oxidize::status;
use renderer::{render, mustache_render};
use collections::hashmap::HashMap;
use std::io::net::ip::SocketAddr;
use std::os::make_absolute;

mod renderer;

#[allow(unused_must_use)]
fn index(request: &Request, response: &mut ResponseWriter, vars: &~[(~str,~str)]) {
    // TODO: maybe make a macro to make this look nicer? But this is faster at least :p
    response.status = status::Ok;
    response.write_content_auto(
        MediaType {type_: ~"text", subtype: ~"html", parameters: vec!()}, 
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
        MediaType {type_: ~"text", subtype: ~"html", parameters: vec!()}, 
        mustache_render("mustache.html", Some(&context)).unwrap()
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
        MediaType{type_: ~"text",subtype: ~"html",parameters: vec!()}, 
        mustache_render("variable.html", Some(&context)).unwrap()
    );
}

// just set this to app's base path relative to the binary location
static APP_PATH : &'static str = "..";

// just set this to the template's dir relative to the app's base path
static TEMPLATE_DIR_NAME : &'static str = "templates";
fn main() {
    let file_name = std::os::args()[0];

    // yeah I know this is flimsy and will break, but its better than befores
    let split : ~[&str] = file_name.rsplitn('/', 1).collect();
    let base_path = split[1] + "/" + APP_PATH;
    let template_path = Path::new(base_path + "/" + TEMPLATE_DIR_NAME);
    let absolute_path = make_absolute(&template_path);
    println!("base_path: {}", absolute_path.display());
    renderer::setup(&absolute_path);

    let routes: ~[TrieRoute] = ~[
        Variable(Route{ method: "GET", name: "index", path: "/", fptr: index}),
        Variable(Route{ method: "GET", name: "test_mustache", path: "/test", fptr: test_mustache}),
        Variable(Route{ method: "GET", name: "test_variable", path: "/users/user-:userid/post-:postid", fptr: test_variable}),
        Static(StaticRoute{ path: "/public/css", directory: "/css"}),
    ];

    // the router must be passed the base path of the app relative to the binary location
    let router = ~TrieRouter::new(routes,base_path);
    let conf = Config {
        debug: true,
        bind_addr: from_str::<SocketAddr>("127.0.0.1:8001").unwrap(),
    };

    let server = Oxidize::new(conf, router as ~Router:Send);
    server.serve();
}
