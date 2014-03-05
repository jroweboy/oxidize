extern crate http;
extern crate mustache;
extern crate serialize;

//use http::server::Request;
use request::Request;

use std::io::File;
use std::str::{from_utf8, from_utf8_owned};
// use collections::hashmap::HashMap;
// use serialize::Encodable;
// use mustache::encoder::{Encoder, Data, Str, Vec, Map};

// #[deriving(Encodable)]
// pub struct Context {
//     data: Hash
// }




#[allow(unused_variable)]
// removed the context from this. This is a file loader for now.
// , context: Option<HashMap<~str,~str>>
pub fn render<'a>(request: &'a Request, file_name: &'a str) -> ~str {
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
// TODO: at compile or start time, I can use mustache to precompile all the templates
// and then just render the template from the precomiled version
// , T: Encodable<Encoder> , context: Option<T>
pub fn mustache_render<'a>(request: &'a Request, file_name: &'a str) -> ~str {
    let path = Path::new("templates/"+file_name);
    println!("Render for this file: {}", path.display());
    let file_contents = File::open(&path).read_to_end().unwrap();
    // TODO: add the request to the context so that they can use things like session vars
    let contents = from_utf8(file_contents).expect("File could not be parsed as UTF8");

    let cxt = request.context.clone();
    if cxt.is_some() {
        mustache::render_str(contents, &cxt.unwrap().clone())
    } else {
        mustache::render_str(contents, &~"")
    }
}