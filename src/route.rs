// extern crate http;

use request::Request;
use http::server::ResponseWriter;
use collections::hashmap::HashMap;
use collections::enum_set::{EnumSet};
use pcre::{CompileOption, StudyOption, ExtraOption, Pcre};
use request;
use pcre;
use sync::RWArc;

pub type View = fn (&Request, &mut ResponseWriter);

pub struct RegexRoute {
	method : &'static str,
	path : &'static str,
	fptr : View,
}

// impl Route for RegexRoute {
impl RegexRoute {
    fn call(&self, request: &Request, response: &mut ResponseWriter) {
        println!("Routing calling the function for path [{}]", self.path);
        (self.fptr)(request, response)
    }
}

// pub trait Route {
//     fn call(&self, request: &Request, response: &mut ResponseWriter);
// }

pub trait Router : Send+Freeze {
    fn route(&self, request: &mut Request, response: &mut ResponseWriter);
    fn reverse(&self, name: &str, vars: Option<HashMap<~str,~str>>) -> Option<&~str>;
}

pub struct RegexRouter {
    routes: &'static [RegexRoute],
    compiled_routes: Pcre,
}

impl Router for RegexRouter {
    fn route(&self, request: &mut request::Request, response: &mut ResponseWriter) {
        // use the massive regex to route
        let uri = request.uri.clone();
        let regex_result = self.compiled_routes.exec(uri);

        // .read (
        //     |re: &Pcre| {re.exec(uri)}
        // );

        let success = match regex_result {
            None => None,
            Some(_) => {
                // get the mark to find the index of the route in the routes
                let mark = self.compiled_routes.get_mark(); //.read (|re: &Pcre| { re.get_mark() });
                println!("Mark problem I bet :p {}", mark);
                // convert the mark to an int and call the appropriate function
                let index = match mark {
                    Some(m) => from_str::<uint>(m),
                    None() => None,
                };
                match index {
                    Some(i) => {self.routes[i].call(request, response); Some("yup")}
                    None => None
                }
            }
        };
        println!("Success! {}", success);
    }

    #[allow(unused_variable)]
    fn reverse(&self, name: &str, vars: Option<HashMap<~str,~str>>) -> Option<&~str> {
        None
    }
}

/// Helper method to build a regex out of all the routes
fn compile_routes(routes : &'static [RegexRoute]) -> Pcre {
    // pure evil unsafeness right here
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
    let mut compiled_routes = //RWArc::<Pcre>::new(
            Pcre::compile_with_options(regex, &compile_options).unwrap();
            //        );

    let mut study_options: EnumSet<StudyOption> = EnumSet::empty();
    study_options.add(pcre::StudyJitCompile);
    compiled_routes.study_with_options(&study_options);
    // .write(
    //     |re: &mut Pcre| { re.study_with_options(&study_options); }
    // );

    // set that I am using the extra mark field
    let mut extra_options: EnumSet<ExtraOption> = EnumSet::empty();
    extra_options.add(pcre::ExtraMark);
    compiled_routes.set_extra_options(&extra_options);
    // .write(
    //     |re: &mut Pcre| { re.set_extra_options(&extra_options); }
    // );
    compiled_routes
}

impl RegexRouter {
    pub fn new(routes: &'static [RegexRoute]) -> RegexRouter {

        RegexRouter {
            routes: routes,
            compiled_routes: compile_routes(routes),
        }
    }
}