// don't think this is used anymore
//extern crate extra;
// libpcre provides regexs for routing
extern crate pcre;
// holds references to HashMap
extern crate collections;
// need to pass the time info to 
extern crate time;
// needed for some Encodable stuff
extern crate serialize;
// templating is provided by rust-mustache
extern crate mustache;
// handles all the http stuff
extern crate http;
// used for holding the pcre struct in a mutable multithreaded way
extern crate sync;


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

use std::cast;
use std::default::Default;
use std::io::net::ip::{SocketAddr, Ipv4Addr};

use response::Response;
use route::{Route};
// TODO use a RWArc its much better since its many reader one writer
// and I'll only be reading almost exclusively
use sync::MutexArc;


pub mod route;
pub mod renderer;
pub mod response;
pub mod request;


// TODO: I hate the idea of routes stored as a mut static
// Solution: I can't use that anymore since statics are not allowed to have destructors anymore

// initialize it to nothing 
// static mut compiled_routes : Option<Pcre> = None;
// maybe make a map from function pointer as the hash and the value be the 
// this checks out below. I can use http://static.rust-lang.org/doc/master/std/cast/fn.transmute.html
// to convert the fn pointer into a raw *() pointer (similar to a void pointer)
// which already has the Hash trait implemented. The reverse function will take in 
// a function pointer and context and return a url? I'm not confident about that one...

// Except for you evil little one. 
// TODO: how can I possibly expose a reverse function without static?
// static mut reverse_routes_str : Option<()> = None;
// HashMap::<*(), &'static str>::new();

#[deriving(Clone)]
pub struct Oxidize {
    // TODO: use this little piece of awesome I found to allow them to choose port and stuff
    //from_str::<SocketAddr>("127.0.0.1:8080").unwrap()
    // http://static.rust-lang.org/doc/0.9/std/io/index.html Found here
    port: u16,
    addr: ~str,
    // TODO: do i even need to store them in here? Looks like it
    routes: &'static [Route<'static>],
    compiled_routes: MutexArc<Pcre>
}



// fn get_compiled_regex() -> &mut Pcre {
//     unsafe { compiled_routes.get_mut_ref() }
// }

// fn get_reverse_route_str() -> &mut HashMap<*(), &'static str> {
//     unsafe { 
//         cast::transmute(reverse_routes_str.get_mut_ref())
//     }
// }

// TODO: move the compiled_routes and the reverse routing everything into route.rs maybe
// pub fn reverse(fptr: route::View) -> &'static str {
//     let fnpointer : &*() = unsafe { cast::transmute(fptr) };
//     *get_reverse_route_str().get(fnpointer)
// }


impl Oxidize {

    pub fn new(p : u16, a : &str, r : &'static [Route<'static>]) -> Oxidize {
        Oxidize {
            port : p,
            addr : a.to_owned(),
            routes : r,
            compiled_routes : Oxidize::compile_routes(r)
        }
    }

    fn route(&self, request: &mut request::Request, response: &mut ResponseWriter) -> ~str {
        // use the massive regex to route
        //println!("request_uri: {}", request.uri.clone());
        // let re = get_compiled_regex();

        // 
        let uri = request.uri.clone();
        let regex_result = self.compiled_routes.access(
            |re: &mut Pcre| {re.exec(uri)}
        );

        // TODO: clean up this crazy massive match tree using functions found in Option
        let resp = match regex_result {
            Some(_) => {
                // get the mark index
                let raw_mark = self.compiled_routes.access(
                    |re: &mut Pcre| { re.get_mark() }
                );
                let index = match raw_mark {
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

        let res = match resp {
            Some(res) => res,
            None => Response {status: status::NotFound, content: ~"404 - Not Found"}
        };

        let reason = res.status.reason();
        let code = res.status.code();

        let newStatus = Status::from_code_and_reason(code,reason);

        response.status = newStatus;
        return res.content;
    }
    /// Builds a giant regex from all of the routes
    fn compile_routes(routes : &'static [Route<'static>]) -> MutexArc<Pcre> {
        // pure evil unsafeness right here
        // unsafe { reverse_routes_str = Some(
        //     // removing the destructor from HashMap so I can store it in a mut static :p
        //         cast::forget::<HashMap<*(), &'static str>>(HashMap::<*(), &'static str>::new())
        // )};
        // let revroute = get_reverse_route_str();
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
            let fnpointer : *() = unsafe { cast::transmute(route.fptr) };
            // revroute.insert(fnpointer, route.path);
            i += 1;
        }
        regex.push_str(")");

        println!("routing regex: {}", regex);

        // set up the regex
        let mut compile_options: EnumSet<CompileOption> = EnumSet::empty();
        compile_options.add(pcre::Extra);
        // TODO: better error handling if unwrap fails on any of these. 
        // I don't think its appropriate to just fail!() either...
        // Maybe an expect explaining the problem would work?
        let compiled_routes = MutexArc::<Pcre>::new(
                Pcre::compile_with_options(regex, &compile_options).unwrap()
            );
        
        // let mut re = self.compiled_routes;
        // let re = get_compiled_regex();

        let mut study_options: EnumSet<StudyOption> = EnumSet::empty();
        study_options.add(pcre::StudyJitCompile);
        compiled_routes.access(
            |re: &mut Pcre| { re.study_with_options(&study_options); }
        );

        // set that I am using the extra mark field
        let mut extra_options: EnumSet<ExtraOption> = EnumSet::empty();
        extra_options.add(pcre::ExtraMark);
        compiled_routes.access(
            |re: &mut Pcre| { re.set_extra_options(&extra_options); }
        );
        compiled_routes
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
        let my_request = &mut request::Request {
            method: test_method, 
            uri: path,
            ..Default::default()
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