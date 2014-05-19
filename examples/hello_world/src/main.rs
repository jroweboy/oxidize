#![feature(phase)]
#[phase(syntax)] 
extern crate oxidize;

#[phase(syntax)] 
extern crate log;

extern crate collections;
extern crate http;
extern crate oxidize;
extern crate log;
extern crate sync;

// use oxidize::route::matchrouter::MatchRouter;
// use oxidize::route::{Router, RouteInfo};
use collections::hashmap::HashMap;
use oxidize::router::Router;
use oxidize::{App, Config, Oxidize, Request, ResponseWriter};
use oxidize::status;
use std::io::net::ip::SocketAddr;
use std::path::posix::Path;
use std::io::File;
use sync::Arc;
use http::method;

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
    fn handle_route<'a>(&self, route: Option<&&'static str>, vars: Option<HashMap<~str,~str>>,
                         req: &mut Request, res: &mut ResponseWriter) {
        if route.is_none() {
            // 404
            self.default_404(req, res);
        } else {
            match *route.unwrap() {
                "index" => {
                    self.index(res);
                }
                "mustache" => {
                    res.write_content_auto(
                        content_type!("text", "html").unwrap(), 
                        StrBuf::from_str("Hello mustache! TODO fix me to actually call the mustache method whenever rust-mustache updates to the latest rust")
                    ); 
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
    fn index(&self, response: &mut ResponseWriter) {
        response.status = status::Ok;
        response.write_content_auto(
            content_type!("text", "html").unwrap(), 
            self.render("index.html")
        );
    }
    fn render(&self, template: &str) -> StrBuf {
        // TODO: Don't let the render function fail! instead properly handle errors and return a Result
        let path = self.template_path.join(Path::new(template));
        debug!("Template Path: {}",  path.display());
        let file_contents = File::open(&path).read_to_end().unwrap();
        StrBuf::from_utf8(file_contents).unwrap()
    }
}

fn main() {
    let app = box HelloWorld {
        template_path: Arc::new(Path::new("/home/jrowe7/slf/oxidize/examples/hello_world/templates")),
    };
    let conf = Config {
        debug: true,
        bind_addr: from_str::<SocketAddr>("127.0.0.1:8001").unwrap(),
    };
    let server = Oxidize::new(conf, app as Box<App:Send+Share>, None);
    server.serve();
}