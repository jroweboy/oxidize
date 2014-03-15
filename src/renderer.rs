extern crate mustache;
extern crate serialize;

use std::io::File;
use std::str::{from_utf8, from_utf8_owned};
use collections::hashmap::HashMap;


//Removed the request for now since its not used request: &'a mut Request, 
pub fn render<'a>(file_name: &'a str) -> ~str {
    // TODO: the templates dir probably shouldn't be hard coded
    let path = Path::new("templates/"+file_name);
    debug!("Render for this file: {}", path.display());
    let file_contents = File::open(&path).read_to_end().unwrap();
    // TODO: add the request to the context so that they can use things like session vars?
    from_utf8_owned(file_contents).expect("File could not be parsed as UTF8")
}

// ditto above request: &'a mut Request, 
pub fn mustache_render<'a>(file_name: &'a str, 
                    context: Option<&'a HashMap<~str,~str>>) -> ~str {
    let path = Path::new("templates/"+file_name);
    debug!("Render for this file: {}", path.display());
    let file_contents = File::open(&path).read_to_end().unwrap();
    // TODO: add the request to the context so that they can use things like session vars
    let contents = from_utf8(file_contents).expect("File could not be parsed as UTF8");

    // TODO: Performance: I don't think I need to clone here
    // TODO: Performance: at compile/first run I should be able to compile and 
    // load all the templates into memory and just render templates?
    if context.is_some() {
        mustache::render_str(contents, &context.unwrap().clone())
    } else {
        mustache::render_str(contents, &~"")
    }
}