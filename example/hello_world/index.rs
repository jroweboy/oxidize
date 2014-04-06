extern crate oxidize;
extern crate collections;
extern crate mustache;

use oxidize::{Oxidize, Request, ResponseWriter, Config, MediaType};
use oxidize::route::Router;
use oxidize::route::trierouter::{TrieRouter, TrieRoute, Route, Simple, Variable};
use oxidize::renderer::render;
use oxidize::status;
use collections::hashmap::HashMap;
use std::io::net::ip::SocketAddr;
use std::io::fs::File;
use std::str::from_utf8;

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

// this should be an absolute path for people making similar apps and end with a '/'
// but for now I can calculate the actual absolute path
static TEMPLATE_DIR : &'static str = "../templates/";
static mut TEMPLATE_PATH : Option<*()> = None;
fn main() {
    // make a absolute directory for this 
    let file_name = std::os::args()[0];
    // yeah I know this is flimsy and will break, but its better than befores
    let split : ~[&str] = file_name.rsplitn('/', 1).collect();
    let path = Path::new(split[1]+ "/" + TEMPLATE_DIR);
    let absolute_path = std::os::make_absolute(&path);
    unsafe { TEMPLATE_PATH = Some(std::cast::transmute(&absolute_path)); }

    let routes: ~[TrieRoute] = ~[
        Simple(Route{ method: "GET", name: "index", path: "/", fptr: index}),
        Simple(Route{ method: "GET", name: "test_mustache", path: "/test", fptr: test_mustache}),
        Variable(Route{ method: "GET", name: "test_variable", path: "/users/user-:userid/post-:postid", fptr: test_variable}),
    ];

    let router = ~TrieRouter::new(routes);
    let conf = Config {
        debug: true,
        template_dir: Some(TEMPLATE_DIR),
        bind_addr: from_str::<SocketAddr>("127.0.0.1:8001").unwrap(),
    };

    let server = Oxidize::new(conf, router as ~Router:Send);
    server.serve();
}

pub fn mustache_render<'a>(file_name: &'a str, 
                    context: Option<&'a HashMap<~str,~str>>) -> Result<~str,mustache::encoder::Error> {
    let path = unsafe { std::cast::transmute::<*(), &Path>(TEMPLATE_PATH.unwrap()) };
    //Path::new(TEMPLATE_DIR+file_name);
    // debug!("Render for this file: {}", path.display());
    let file_contents = File::open(path).read_to_end().unwrap();
    // TODO: add the request to the context so that they can use things like session vars
    let contents = from_utf8(file_contents).expect("File could not be parsed as UTF8");

    // TODO: Performance: I don't think I need to clone here
    if context.is_some() {
        mustache::render_str(contents, &context.unwrap().clone())
    } else {
        mustache::render_str(contents, &~"")
    }
}