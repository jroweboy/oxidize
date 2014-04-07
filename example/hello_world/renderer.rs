extern crate mustache;
extern crate serialize;

use std::io::File;
use std::str::{from_utf8_owned};
use std::cast::transmute;
use collections::hashmap::HashMap;

pub fn setup<'a>(absolute_path : &'a Path) {
    unsafe { TEMPLATE_PATH = Some(transmute::<&Path,*()>(absolute_path)); }
}

static mut TEMPLATE_PATH : Option<*()> = None;
pub fn render<'a>(file_name: &'a str) -> ~str {
    let base_path = unsafe { transmute::<*(), &Path>(TEMPLATE_PATH.unwrap()) };
    let path = base_path.join(Path::new(file_name));
    let file_contents = File::open(&path).read_to_end().unwrap();
    // TODO: add the request to the context so that they can use things like session vars?
    from_utf8_owned(file_contents).expect("File could not be parsed as UTF8")
}

pub fn mustache_render<'a>(file_name: &'a str, 
                    context: Option<&'a HashMap<~str,~str>>) -> Result<~str,mustache::encoder::Error> {
    // let base_path = unsafe { transmute::<*(), &Path>(TEMPLATE_PATH.unwrap()) };
    // let path = base_path.join(Path::new(file_name));
    // let file_contents = File::open(&path).read_to_end().unwrap();
    // // TODO: add the request to the context so that they can use things like session vars
    // let contents = from_utf8(file_contents).expect("File could not be parsed as UTF8");

    // TODO: Performance: I don't think I need to clone here
    if context.is_some() {
        mustache::render_str(render(file_name), &context.unwrap().clone())
    } else {
        mustache::render_str(render(file_name), &~"")
    }
}