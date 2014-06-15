#![feature(phase)]
#[phase(syntax)] 
extern crate log;
#[phase(syntax)]
extern crate oxidize_macros;

extern crate oxidize;
extern crate collections;
extern crate log;
extern crate sync;

use oxidize::router::Router;
use oxidize::common::{method};
use oxidize::{App, Config, Oxidize, Request, Response};
use oxidize::backend::rusthttp::RustHttpBackend;
use oxidize::backend::OxidizeBackend;

use std::collections::HashMap;
use std::path::posix::Path;
use std::io::File;
use sync::Arc;

// One of my goals is to make a macro that takes this and impls App for MyApp like below
// routes!{HelloWorld,
//     ("GET", "/", "index", self.index)
//     ("GET", "/mustache", "mustache", self.test_mustache),
//     ("GET", "/users/user-<userid:uint>/post-<postid:uint>", "test_variable", self.test_variable),
//     fn default_404(&self, &mut Request, &mut ResponseWriter) {
//         404 code if you want to override the default 404 page
//     }
// }

impl App for HelloWorld {
    #[allow(unused_must_use)]
    #[allow(unused_variable)]
    fn handle_route<'a>(&self, route: Option<&&'static str>, vars: Option<HashMap<String,String>>,
                         req: &mut Request) -> Response {
        if route.is_none() {
            // 404
            return self.default_404(req);
        } else {
            match *route.unwrap() {
                "index" => {
                    self.index()
                }
                "mustache" => {
                    Response::empty()
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }
    fn get_router(&self) -> Router<&'static str> {
        let routes = [
            (method::Get, "/", "index"),
            (method::Get, "/mustache", "mustache"),
        ];
        Router::from_routes(routes)
    }
}


struct HelloWorld {
    template_path: Arc<Path>,
}

impl HelloWorld {
    #[allow(unused_must_use)]
    fn index(&self) -> Response {
        Response::ok(self.render("index.html"), None)
    }

    fn render(&self, template: &str) -> String {
        // TODO: Don't let the render function fail! instead properly handle errors and return a Result
        // Or maybe return a 500 error message? I'm not sure what the proper thing should be
        let path = self.template_path.join(Path::new(template));
        let file_contents = File::open(&path).read_to_end().unwrap();
        String::from_utf8(file_contents).unwrap()
    }
}

fn main() {
    let app = box HelloWorld {
        // a hard coded path to the template directory. Change that to be your template location
        template_path: Arc::new(Path::new("/home/jrowe7/slf/oxidize/examples/hello_world/templates")),
    };
    // the Config currently does nothing. Its kinda there to reserve a space for something useful later.
    let conf = Config {
        debug: true,
    };
    let oxidize = Oxidize::new(conf, app as Box<App:Send+Share>, None);
    let server = RustHttpBackend::new("127.0.0.1:8001", oxidize);
    server.serve();
}