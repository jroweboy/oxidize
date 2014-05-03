#![feature(phase)]
#[phase(syntax)] 
extern crate oxidize;

extern crate oxidize;
extern crate sync;

use oxidize::route::MatchRouter;
use oxidize::{App, Config, Oxidize, Request, ResponseWriter, MediaType};
use oxidize::status;

routes!{HelloWorld, MatchRouter,
    ("GET", "/", "index", self.index)
}

    // ("GET", "/test", "test_mustache", self.test_mustache),
    // ("GET", "/users/user-<userid:uint>/post-<postid:uint>", "test_variable", self.test_variable),


struct HelloWorld {
    template_path: Arc<RWLock<StrBuf>>,
}

impl HelloWorld {
    fn index(&self, request: &mut Request, response: &mut ResponseWriter) {
        response.status = status::Ok;
        response.write_content_auto(
            content_type!("text", "html"), 
            self.render("index.html")
        );
    }

    fn test_mustache(&self, request: &mut Request, response: &mut ResponseWriter) {

    }

    // fn test_variable(&self, request: &mut Request, response: &mut ResponseWriter, userid: uint, postid: uint) {
        
    // }

    fn render(&self, template: &str) -> StrBuf {
        let path = self.template_path.join(Path::new(template));
        let file_contents = File::open(&path).read_to_end().unwrap();
        StrBuf::from_utf8(file_contents).expect("File could not be parsed as UTF8")
    }

    // fn mustache(&self, template: &str,
    //         context: Option<&'a HashMap<~str,~str>>) -> Result<~str,mustache::encoder::Error> {
    //     if context.is_some() {
    //         mustache::render_str(self.render(file_name), &context.unwrap().clone())
    //     } else {
    //         mustache::render_str(self.render(file_name), &~"")
    //     }
    // }
}

fn main() {
    let app = HelloWorld {
        template_path: StrBuf::from_str("/home/james/slf/oxidize/example/hello_world/templates"),
    };
    let conf = Config {
        debug: true,
        bind_addr: from_str::<SocketAddr>("127.0.0.1:8001").unwrap(),
    };

    let server = Oxidize::new(conf, app.get_router(), app);
    server.serve();
}