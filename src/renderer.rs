extern crate http;
extern crate mustache;
extern crate serialize;

//use http::server::Request;
use request::Request;

use std::io::File;
use std::str::{from_utf8, from_utf8_owned};
use collections::hashmap::HashMap;
// use serialize::Encodable;
// use mustache::encoder::{Encoder, Data, Str, Vec, Map};

// #[deriving(Encodable)]
// pub struct Context {
//     data: Hash
// }




#[allow(unused_variable)]
// removed the context from this. This is a file loader for now.
// , context: Option<HashMap<~str,~str>>
pub fn render<'a>(request: &'a mut Request, file_name: &'a str, 
                    context: Option<&'a HashMap<~str,~str>>) -> ~str {
    // TODO: the templates dir probably shouldn't be hard coded
    // let path = Path::new("templates/"+file_name);
    // println!("Render for this file: {}", path.display());
    // let contents = File::open(&path).read_to_end();

    // match from_utf8(contents.unwrap()) {
    //     Some(str) => str,
    //     None => fail!("File could not be rendered")
    // }
    let path = Path::new("templates/"+file_name);
    println!("Render for this file: {}", path.display());
    let file_contents = File::open(&path).read_to_end().unwrap();
    // TODO: add the request to the context so that they can use things like session vars
    from_utf8_owned(file_contents).expect("File could not be parsed as UTF8")
}

#[allow(unused_variable)]
// , T: Encodable<Encoder> , context: Option<T>
pub fn mustache_render<'a>(request: &'a mut Request, file_name: &'a str, 
                    context: Option<&'a HashMap<~str,~str>>) -> ~str {
    let path = Path::new("templates/"+file_name);
    println!("Render for this file: {}", path.display());
    let file_contents = File::open(&path).read_to_end().unwrap();
    // TODO: add the request to the context so that they can use things like session vars
    let contents = from_utf8(file_contents).expect("File could not be parsed as UTF8");

    // TODO: Performance: I don't think I need to clone here
    // TODO: Performance: at compile/first run I should be able to compile and 
    // load all the templates into memory and just render templates
    //let cxt = context.clone();
    if context.is_some() {
        mustache::render_str(contents, &context.unwrap().clone())
    } else {
        mustache::render_str(contents, &~"")
    }
}