use collections::hashmap::HashMap;
use collections::smallintmap::SmallIntMap;
use request;
use mimetype;
use status;
use status::Status;
use sync::Arc;
use route::Router;
use request::Request;
use http::server::ResponseWriter;
use std::io;
use std::io::File;
use http::method;

//TODO: ensure routes are valid URLs
// allowed url characters: $-_.+!*'(),
// reserved characters: $&+,/:;=?@
// % is used for escaped characters
// # is used for anchors
// ~ is often used in urls even though it is "unsafe"

pub type View = fn (&Request, &mut ResponseWriter, &~[(~str,~str)]);

pub struct TrieRouter<'a> {
    priv trie: Arc<TrieRouterNode>,
    priv reverse_routes: Arc<HashMap<~str, ~str>>,
}

impl<'a> Router for TrieRouter<'a> {
    fn route(&self, request: &mut request::Request, response: &mut ResponseWriter) {
        let routing_info = self.get_routing_info(request.uri);
        match routing_info {
            Handler(fptr,vars) => {
                (fptr)(request, response, &vars);
            },
            StaticPath(dir,file) => {
                let path = dir+"/"+file;
                self.serve_static_file(path, request, response);
            },
            Error(status) => {
                self.error_response(request.uri.to_owned(),status, response);
            },
        }
    }
    #[allow(unused_variable)]
    fn reverse<'a>(&'a self, name: &str, vars: Option<~[(~str,~str)]>) -> Option<~str> {
        match vars {
            Some(list) => self.reverse_routes.get().find_equiv(&name).and_then(
                |path: &'a ~str| {
                    let mut original_path = path.clone();
                    let mut new_path: ~str;
                    for var in list.iter() {
                        let t = var.clone();
                        let (mut k,v) = t;
                        k.unshift_char(':');
                        new_path = original_path.replace(k,v);
                        original_path = new_path.clone();
                    }
                    Some(original_path)
                }
            ),
            None => self.reverse_routes.get().find_equiv(&name).and_then(
                |path: &'a ~str| {
                    Some(path.to_owned())
                }
            )
        }
        
    }

    fn copy(&self) -> ~Router {
        ~TrieRouter {
            trie: self.trie.clone(),
            reverse_routes: self.reverse_routes.clone(),
        } as ~Router
    }
}

impl<'a> TrieRouter<'a> {
    pub fn new(routes: ~[TrieRoute]) -> TrieRouter {
        let mut reverse_routes = HashMap::<~str,~str>::new();
        for route in routes.iter() {
            match *route {
                Variable(r) => {reverse_routes.insert(r.name.to_owned(),r.path.to_owned());},
                Static(_) => {}
            }
        }
        let trie = TrieRouter::build_routing_trie(&routes);
        TrieRouter {
            trie: Arc::new(trie),
            reverse_routes: Arc::new(reverse_routes)
        }
    }

    pub fn get_routing_info<'a>(&'a self, uri: &str) -> RoutingInfo {
        let mut path = uri.to_owned();

        if path.len() != 1 && path[path.len() - 1] == '/' as u8 {
            path.pop_char();
        }

        let mut current_node = self.trie.get();
        let mut current_var = ~"";
        let mut current_key = ~"";
        let mut building_var = false;
        let mut not_found = false;
        let mut vars = ~[];

        for ch in path.chars() {
            if building_var && ch == '/' {
                vars.push((current_key.clone(),current_var.clone()));
                building_var = false;
                if current_node.children.contains_key(&(ch as uint)) {
                    current_node = current_node.children.get(&(ch as uint));
                }
                else {
                    not_found = true;
                    break;
                }
            }
            else if building_var {
                current_var.push_char(ch);
            }
            // putting this else block before the next one allows routes to "clash"
            // e.g. route1: "/blog/post:id"
            // e.g. route1: "/blog/posts"
            // could both exist but "s" would never be the "id"
            else if current_node.children.contains_key(&(ch as uint)) {
                current_node = current_node.children.get(&(ch as uint));
            }
            else if current_node.children.contains_key(&(':' as uint)) {
                current_node = current_node.children.get(&(':' as uint));
                // cloning here because it tried to move varname but the current_node 
                // is only a non mutable pointer here so it can't move.
                current_key = current_node.varname.clone().unwrap_or(~"");
                current_var.truncate(0);
                current_var.push_char(ch);
                building_var = true;
            }
            else {
                not_found = true;
                break;
            }
        }
        if building_var {
            vars.push((current_key.clone(),current_var.clone()));
        }

        if not_found || (current_node.fptr.is_none() && current_node.staticdir.is_none()) {
            Error(status::NotFound)
        }
        else if current_node.fptr.is_none() {
            StaticPath(current_node.staticdir.clone().unwrap(),current_var)
        }
        else {
            Handler(current_node.fptr.unwrap(),vars)
        }
    }

    fn build_routing_trie(routes : &~[TrieRoute]) -> TrieRouterNode {
        let mut root = TrieRouterNode::new();

        for route in routes.iter() {
            match *route {
                Variable(r) => {root.add(r);},
                Static(r) => {root.add_static(r);}
            }
        }
        root
    }

    fn serve_static_file(&self, path: ~str, request: &mut Request, response: &mut ResponseWriter) {
        println!("{}","serve_static_file");
        match request.method {
            method::Get => {
                let mut file = File::open(&Path::new("."+path));
                let mut result = file.read_to_end();
                match result {
                    Ok(_) => {
                        let pieces: ~[&str] = path.rsplitn('.',1).collect();
                        let extension = pieces[0].to_owned();
                        let content_type = mimetype::content_type_from_ext(extension);
                        response.headers.content_type = Some(content_type);
                        response.write(result.unwrap());
                    },
                    Err(err) => {
                        match err.kind {
                            io::FileNotFound => self.error_response(path, status::NotFound, response),
                            io::PermissionDenied => self.error_response(path, status::Forbidden, response),
                            _ => self.error_response(path, status::InternalServerError, response),
                        }
                    },
                }
            },
            _ => self.error_response(path, status::MethodNotAllowed, response),
        }
    }

    fn error_response(&self, uri: ~str, status: Status, response: &mut ResponseWriter) {
        //TODO: implement error responses and custom error responses
        println!("{} {} {}", uri, status.code(), status.reason());
    }
}

pub enum TrieRoute {
    Variable(Route),
    Static(StaticRoute),
}

enum RoutingInfo {
    Error(Status),
    Handler(View,~[(~str,~str)]),
    StaticPath(~str,~str),
}

pub struct Route {
    method: &'static str,
    path : &'static str,
    name : &'static str,
    fptr : View,
}

pub struct StaticRoute {
    path: &'static str,
    directory: &'static str,
}

struct TrieRouterNode {
    children: SmallIntMap<TrieRouterNode>,
    fptr: Option<View>,
    varname: Option<~str>,
    staticdir: Option<~str>,
}

impl TrieRouterNode {
    pub fn new() -> TrieRouterNode {
        TrieRouterNode {
            children : SmallIntMap::new(),
            fptr: None,
            varname: None,
            staticdir: None,
        }
    }

    fn add(&mut self, route: Route) {
        let mut current_var = ~"";
        let mut building_var = false;
        let path = route.path;

        if path.len() == 0 {
            // warn!("route path must not be empty")
        }
        if path[0] != '/' as u8 {
            // error: route path must begin with '/'
        }
        if path.len() != 1 && path[path.len()-1] == '/' as u8 {
            // error: route path must not end with '/'
        }

        {
            let mut current_node = self;
            for ch in path.chars() {
                if building_var && ch == '/' {
                    if current_var.len() == 0 {
                        // error: can't allow empty string as varname
                    }
                    let tmp = current_node;
                    let tmp2 = tmp.children.find_or_insert(':', TrieRouterNode::new());
                    tmp2.varname = Some(current_var.clone());
                    current_node = tmp2.children.find_or_insert(ch, TrieRouterNode::new());
                    building_var = false;
                }
                else if building_var {
                    current_var.push_char(ch);
                }
                else if ch == ':' {
                    building_var = true;
                    current_var.truncate(0);
                }
                else {
                    let tmp = current_node;
                    current_node = tmp.children.find_or_insert(ch, TrieRouterNode::new());
                }
            }
            if building_var {
                let tmp = current_node;
                let tmp2 = tmp.children.find_or_insert(':', TrieRouterNode::new());
                tmp2.varname = Some(current_var);
                current_node = tmp2;
            }
            current_node.fptr = Some(route.fptr);
        }
    }

    fn add_static(&mut self, route: StaticRoute) {
        let path = route.directory;

        if path.len() == 0 {
            // warn!("route path must not be empty")
        }
        if path[0] != '/' as u8 {
            // warn!("route path must begin with '/'");
        }
        if path.len() != 1 && path[path.len()-1] == '/' as u8 {
            // warn!("route path must not end with '/'");
        }

        {
            let mut current_node = self;
            for ch in path.chars() {
                let tmp = current_node;
                current_node = tmp.children.find_or_insert(ch, TrieRouterNode::new());
            }
            let tmp = current_node;
            let tmp2 = tmp.children.find_or_insert('/', TrieRouterNode::new());
            let tmp = tmp2.children.find_or_insert(':', TrieRouterNode::new());
            current_node = tmp;
            current_node.varname = Some(~"filename");
            current_node.staticdir = Some(route.path.to_owned());
        }
    }
}

// HashMap has this great find_or_insert function
// but SmallIntMap doesn't, so I it
pub trait FindOrInsert {
    fn find_or_insert<'a>(&'a mut self, ch: char, node: TrieRouterNode) -> &'a mut TrieRouterNode;
}

impl FindOrInsert for SmallIntMap<TrieRouterNode> {
    fn find_or_insert<'a>(&'a mut self, ch: char, node: TrieRouterNode) -> &'a mut TrieRouterNode {
        let c : uint = ch as uint;
        if self.contains_key(&c) {
            self.find_mut(&c).unwrap()
        }
        else {
            self.insert(c, node);
            self.find_mut(&c).unwrap()
        }
    }
}