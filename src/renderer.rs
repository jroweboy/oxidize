// extern crate mustache;
extern crate serialize;

use std::io::File;
use std::str::{from_utf8_owned};


// My goal is to move this to a user side thing that they can link against maybe
// so until then I have this ugly hack in place to load the template dir
// static mut TEMPLATE_DIR : [u8, ..255] = [0, ..255];
pub static mut TEMPLATE_DIR : &'static str = "";
pub fn render<'a>(file_name: &'a str) -> ~str {
    // TODO: the templates dir probably shouldn't be hard coded
    let path = Path::new(unsafe{TEMPLATE_DIR+file_name});
    debug!("Render for this file: {}", path.display());
    let file_contents = File::open(&path).read_to_end().unwrap();
    // TODO: add the request to the context so that they can use things like session vars?
    from_utf8_owned(file_contents).expect("File could not be parsed as UTF8")
}