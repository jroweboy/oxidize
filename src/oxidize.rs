extern crate http;
extern crate extra;
extern crate pcre;
extern crate collections;
extern crate time;
extern crate serialize;

// It turns out its real easy to reexport mods :D
// People that extern mod oxidize can use oxidize::reexported::mod;
pub use http::status;
//pub use http::server::Request;
pub use request::Request;
pub use response::Response;
pub use route::Route;

// I'm explicitly including http::server::Request right now
// since I have a struct also named Request
use http::server::{Config, Server, ResponseWriter}; 
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};
use http::headers;
use http::status;
use http::status::Status;
use collections::enum_set::{EnumSet};
use collections::hashmap::HashMap;
use pcre::{CompileOption, StudyOption, ExtraOption, Pcre};

use std::io::net::ip::{SocketAddr, Ipv4Addr};

use response::Response;
use route::{Route};


pub mod route;
pub mod renderer;
pub mod response;
pub mod request;

// initialize it to nothing 
static mut compiled_routes : Option<Pcre> = None;
// maybe make a map from function pointer as the hash and the value be the 
// this checks out below. I can use http://static.rust-lang.org/doc/master/std/cast/fn.transmute.html
// to convert the fn pointer into a raw *() pointer (similar to a void pointer)
// which already has the Hash trait implemented. The reverse function will take in 
// a function pointer and context and return a url? I'm not confident about that one...
//static mut reverse_routes : Option<HashMap<*(), &'static str>> = None;
// HashMap::<*(), &'static str>::new();

#[deriving(Clone)]
pub struct Oxidize {
    // TODO: use this little piece of awesome I found to allow them to choose port and stuff
    //from_str::<SocketAddr>("127.0.0.1:8080").unwrap()
    // http://static.rust-lang.org/doc/0.9/std/io/index.html Found here
    port: u16,
    addr: ~str,
    routes: &'static [Route<'static>],
}

/// Builds a giant regex from all of the routes
fn compile_routes(routes : &'static [Route<'static>]) {
    let mut map = HashMap::new();
    let mut regex = ~"(?";
    let mut i : u32 = 0;
    for route in routes.iter() {
        regex.push_str("|");
        // TODO add the method to the regex
        //regex.push_str(route.method.to_owned());
        regex.push_str(route.path.to_owned());
        regex.push_str("(*MARK:");
        regex.push_str(i.to_str());
        regex.push_str(")");
        map.insert(i.to_str(), route.fptr);
        i += 1;
    }
    regex.push_str(")");

    println!("routing regex: {}", regex);

    // set up the regex
    let mut compile_options: EnumSet<CompileOption> = EnumSet::empty();
    compile_options.add(pcre::Extra);
    unsafe {
        compiled_routes = Some(Pcre::compile_with_options(regex, &compile_options).unwrap());
    }

    let re = get_compiled_regex();

    let mut study_options: EnumSet<StudyOption> = EnumSet::empty();
    study_options.add(pcre::StudyJitCompile);
    re.study_with_options(&study_options);

    // set that I am using the extra mark field
    let mut extra_options: EnumSet<ExtraOption> = EnumSet::empty();
    extra_options.add(pcre::ExtraMark);
    re.set_extra_options(&extra_options);
}

fn get_compiled_regex() -> &mut Pcre {
    unsafe { compiled_routes.get_mut_ref() }
}


impl Oxidize {

    pub fn new(p : u16, a : &str, r : &'static [Route<'static>]) -> Oxidize {
        compile_routes(r);
        Oxidize {
            port : p,
            addr : a.to_owned(),
            routes : r,
        }
    }

    // TODO: Shouldn't route just take in a request?
    fn route(&self, request: request::Request, response: &mut ResponseWriter) -> ~str {
        // use the massive regex to route
        //println!("request_uri: {}", request.uri.clone());

        let re = get_compiled_regex();
        let resp = match re.exec(request.uri.clone()) {
            Some(_) => {
                // get the mark index
                let index = match re.get_mark() {
                    // and convert the string to an int
                    Some(m) => {println!("MARK: {}",m); from_str::<int>(m)},
                    None => None
                };
                println!("Route Index: {}", index);
                // if we got an int then we can use that as the index in the route array
                match index {
                    Some(i) => Some(self.routes[i].call(request)),
                    None => None
                }
            },
            None => None
        };

        let res : ~Response = match resp {
            Some(res) => res,
            None => ~Response {status: status::NotFound, content: ~"404 - Not Found"}
        };

        let reason = res.status.reason();
        let code = res.status.code();

        let newStatus = Status::from_code_and_reason(code,reason);

        response.status = newStatus;
        return res.content;
    }

    // The Server trait has serve_forever as private, so this is my hackish way to expose it
    // TODO maybe ask upstream to change that?
    pub fn serve(self) {
        let addr = self.get_config().bind_address;
        println!("Server is now running at {}", addr.to_str());
        self.serve_forever();
    }
}

#[allow(unused_must_use)]
impl Server for Oxidize {

    fn get_config(&self) -> Config {
        // TODO: Read the data and better handle user data (see the struct def)
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: self.port } }
    }

    fn handle_request(&self, req: &http::server::Request, res: &mut ResponseWriter) {
        res.headers.date = Some(time::now_utc());
        res.headers.server = Some(~"Oxidize/0.0.0 (Ubuntu)");

        // create a request object
        let path = match req.request_uri {
            AbsolutePath(ref i) => i.to_str(),
            AbsoluteUri(ref i) => i.to_str(),
            Authority(ref i) => i.to_str(),
            Star => ~"error" // ?
        };
        let test_method = match from_str("GET") {
            Some(m) => m,
            None => http::method::Get
        };
        let my_request = request::Request {
            method: test_method, 
            uri: path,
            GET: None,
            POST: None
        };
        let response_body = self.route(my_request,res);

        res.headers.content_type = Some(headers::content_type::MediaType {
            type_: ~"text",
            subtype: ~"html",
            parameters: ~[]
        });

        res.headers.content_length = Some(response_body.len());
        res.write(response_body.as_bytes());
    }
}