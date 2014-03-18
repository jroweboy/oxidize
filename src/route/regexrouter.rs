use collections::hashmap::HashMap;
use collections::enum_set::{EnumSet};
use pcre::{CompileOption, StudyOption, ExtraOption, Pcre, Match};
use http::headers::content_type::MediaType;
use std::io::File;
use request;
use pcre;
use http::status;
use sync::{RWArc, Arc};
use route::Router;
use request::Request;
use http::server::ResponseWriter;


pub struct RegexRouter<'a> {
    routes: RWArc<~[RegexRoute]>,
    reverse_routes: Arc<HashMap<~str, ~str>>,
    compiled_routes: RWArc<Pcre>,
}

impl<'a> Router for RegexRouter<'a> {
    fn route(&self, request: &mut request::Request, response: &mut ResponseWriter) {
        // use the massive compiled regex to route
        let uri = request.uri.clone();
        let regex_result = self.compiled_routes.write(|re: &mut Pcre| {re.exec(uri)});
        // Get the mark on the match object and change it to an uint 
        // then find the route at that uint and call the fptr for it
        regex_result.and_then::<Option<uint>>(|rr: Match| {
            rr.mark.and_then(|m: ~str| {from_str::<uint>(m)}).and_then(|i: uint| {
                self.routes.read(|x: &~[RegexRoute]| { 
                    match x[i] {
                        Simple(s) => {(s.fptr)(request, response);},
                        Regex(s) => {(s.fptr)(request, response);},
                        Static(_) => {debug!("TODO: impl StaticServe");},
                    };
                    None
                })
            })
        });
    }

    #[allow(unused_variable)]
    fn reverse<'a>(&'a self, name: &str, vars: Option<HashMap<~str,~str>>) -> Option<&'a ~str> {
        // TODO: use the vars to replace regex things 
        // TODO: remove all the ugly regexy stuff to make a valid URL
        self.reverse_routes.get().find_equiv(&name).and_then(
            |path: &'a ~str| { Some(path) }
        )
    }

    fn copy(&self) -> ~Router {
        ~RegexRouter {
            routes: self.routes.clone(),
            compiled_routes: self.compiled_routes.clone(),
            reverse_routes: self.reverse_routes.clone(),
        } as ~Router
    }
}

impl<'a> RegexRouter<'a> {
    pub fn new(routes: ~[RegexRoute]) -> RegexRouter {
        let mut rev_map = HashMap::<~str,~str>::new();
        for route in routes.iter() {
            match *route {
                Simple(r) => {rev_map.insert(r.name.to_owned(),r.convert_simple_regex().to_owned());},
                Regex(r) => {rev_map.insert(r.name.to_owned(),r.path.to_owned());},
                Static(_) => {debug!("TODO: should Serve have a reverse?");}
            };
        }
        let pcre = RWArc::new(compile_routes(&routes));
        RegexRouter {
            routes: RWArc::new(routes),
            compiled_routes: pcre,
            reverse_routes: Arc::new(rev_map),
        }
    }
}

/// Helper method to build a regex out of all the routes
fn compile_routes(routes : &~[RegexRoute]) -> Pcre {
    let mut regex = ~"(?";
    let mut i : u32 = 0;
    for route in routes.iter() {
        let route_regex = match *route {
            Simple(s) => { Some(s.convert_simple_regex()) },
            Regex(s) => { Some(s.path.to_owned()) },
            Static(_) => {debug!("TODO: impl StaticServe"); None},
        };
        // ignore this route if its not implemented yet
        if route_regex.is_none() {
            continue;
        }

        regex.push_str("|");
        // TODO: add the method to the regex
        //regex.push_str(route.method.to_owned());
        let rr = route_regex.unwrap().to_owned();
        if !rr.starts_with("^") {
            regex.push_char('^');
        }
        regex.push_str(rr);
        if !rr.ends_with("$") {
            regex.push_char('$');
        }
        regex.push_str("(*MARK:");
        regex.push_str(i.to_str());
        regex.push_char(')');
        i += 1;
    }
    regex.push_char(')');

    debug!("routing regex: {}", regex);

    // set up the regex
    let mut compile_options: EnumSet<CompileOption> = EnumSet::empty();
    compile_options.add(pcre::Extra);
    compile_options.add(pcre::DupNames);
    // TODO: better error handling if unwrap fails on any of these. 
    // I don't think its appropriate to just fail!() either...
    // Maybe an expect explaining the problem would work?
    let mut compiled_routes = match Pcre::compile_with_options(regex, &compile_options) {
        Ok(s) => s,
        Err(s) => {debug!("{}", s.message()); fail!("Halting");}
    };

    let mut study_options: EnumSet<StudyOption> = EnumSet::empty();
    study_options.add(pcre::StudyJitCompile);
    compiled_routes.study_with_options(&study_options);

    // set that I am using the extra mark field
    let mut extra_options: EnumSet<ExtraOption> = EnumSet::empty();
    extra_options.add(pcre::ExtraMark);
    compiled_routes.set_extra_options(&extra_options);

    compiled_routes
}

pub type View = fn (&Request, &mut ResponseWriter);

pub enum RegexRoute {
    Simple(Route),
    Regex(Route),
    Static(Serve),
    // ImportRoutes(ImportRoutes),
}

pub struct Route {
    method: &'static str,
    path : &'static str,
    name : &'static str,
    fptr : View,
}

pub struct Serve {
    method: &'static str,
    path: &'static str,
    directory: &'static str, 
}


// TODO: make a way to import routes from another app
// IDEA maybe make a way to package a library version of your app
// and then they can import from your library and construct your library and 
// call things from it and stuff. I'll look into that.
// struct ImportRoutes { 
//     prefix: &'static str,
//     // The issue is that apps make their Router in main which isn't accessible from elsewhere
//     router: RefCell<~Router>,
// }

// static MediaTypeImageJPG : MediaType = 
//     MediaType {type_: "image", subtype: "jpg", parameters: &'static []};
// static MediaTypeTextHTML : 
//     MediaType = MediaType {type_: "text", subtype: "html", parameters: &'static []};
impl Serve { 
    #[allow(dead_code)]
    fn get_star_regex(&self) -> ~str {
        let mut regex = ~"^";
        regex.push_str(self.path.replace("*", ".+"));
        regex.push_char('?');
        regex
    }

    #[allow(dead_code)]
    #[allow(unused_must_use)]
    fn load_file(path: &str, r: &mut ResponseWriter) {
        debug!("Render for this file: {}", path);
        let file_path = Path::new(path);
        match File::open(&file_path).read_to_end() {
            Ok(s) => {
                r.status = status::Ok;
                r.headers.content_type = 
                    Some(MediaType {type_: ~"image", subtype: ~"jpg", parameters: ~[]});
                r.headers.content_length = Some(s.len());
                r.write(s);
            },
            Err(_) => {
                r.status = status::NotFound;
                r.write_content_auto(
                    MediaType {type_: ~"text", subtype: ~"html", parameters: ~[]}, 
                    ~"404"
                );
            }
        };
    }
}

impl Route {
    #[allow(uppercase_variables)]
    fn convert_simple_regex(&self) -> ~str {
        let VARIABLE = 1;
        let NORMAL = 0;
        let mut regex = ~"^";
        let mut state = NORMAL;
        for c in self.path.chars() {
            // a very cruddy finite state machine
            // TODO: escape any regex special characters
            if c == ':' && state == NORMAL {
                state = VARIABLE;
                regex.push_str("(?P<");
            } else if c == ':' && state == VARIABLE {
                fail!("cannot parse the path for {}", self.path);
            } else if c == '/' && state == VARIABLE {
                state = NORMAL;
                regex.push_str(">.+)/");
            } else {
                regex.push_char(c);
            }
        }
        // check to see if they ended with a var /test/:id
        if state == VARIABLE {
            regex.push_str(">.+)/");
        }
        // make the trailing / optional
        if regex.char_at(regex.len() - 1) == '/' {
            regex.push_char('?');
        } else {
            regex.push_str("/?");
        }
        //TODO: (with all routes) fix the GET params situation for routing
        // aka /test/?blah=blah does not route currently
        regex.push_char('$');
        regex
    }
}