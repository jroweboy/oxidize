use collections::hashmap::HashMap;
use collections::smallintmap::SmallIntMap;
use std::clone::Clone;
use request;
use sync::Arc;
use route::Router;
use request::Request;
use http::server::ResponseWriter;

//TODO: ensure routes are valid URLs
// allowed url characters: $-_.+!*'(),
// reserved characters: $&+,/:;=?@
// % is used for escaped characters
// # is used for anchors
// ~ is often used in urls even though it is "unsafe"
// what should be the character used to represent a variable in the built tree?
// maybe ^
// what should be the character to act as a delimeter for variable when defining a route

pub type View = fn (&Request, &mut ResponseWriter, &~[(~str,~str)]);

pub struct TrieRouter<'a> {
    trie: Arc<TrieRouterNode>,
    reverse_routes: Arc<HashMap<~str, ~str>>
}

impl<'a> Router for TrieRouter<'a> {
    fn route(&self, request: &mut request::Request, response: &mut ResponseWriter) {
        let (handler,vars) = self.get_handler(request.uri);
        match handler {
            Some(fptr) => (fptr)(request, response, &vars),
            None => debug!("Implement 404 for TrieRouter")
        }
    }
    #[allow(unused_variable)]
    fn reverse<'a>(&'a self, name: &str, vars: Option<~[(~str,~str)]>) -> Option<~str> {
        match vars {
            Some(list) => self.reverse_routes.find_equiv(&name).and_then(
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
            None => self.reverse_routes.find_equiv(&name).and_then(
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
                Simple(r) => {reverse_routes.insert(r.name.to_owned(),r.path.to_owned());},
                Variable(r) => {reverse_routes.insert(r.name.to_owned(),r.path.to_owned());}
            }
        }
        let trie = TrieRouter::build_routing_trie(&routes);
        TrieRouter {
            trie: Arc::new(trie),
            reverse_routes: Arc::new(reverse_routes)
        }
    }

    pub fn get_handler<'a>(&'a self, uri: &str) -> (Option<View>,~[(~str,~str)]) {
        let mut path = uri.to_owned();

        if path.len() != 1 && path[path.len() - 1] == '/' as u8 {
            path.pop_char();
        }

        let mut current_node : &TrieNode = self.trie.deref();
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
            else if current_node.children.contains_key(&('^' as uint)) {
                current_node = current_node.children.get(&('^' as uint));
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

        if not_found {
            (None, vars)
        }
        else {
            (current_node.fptr, vars)
        }
    }

    fn build_routing_trie(routes : &~[TrieRoute]) -> TrieRouterNode {
        let mut root = TrieRouterNode::new();

        for route in routes.iter() {
            match *route {
                Simple(r) => {root.add(r);},
                Variable(r) => {root.add_variable(r);},
            }
        }
        root
    }
}

pub enum TrieRoute {
    Simple(Route),
    Variable(Route)
}

pub struct Route {
    pub method: &'static str,
    pub path : &'static str,
    pub name : &'static str,
    pub fptr : View,
}

struct TrieRouterNode {
    children: SmallIntMap<TrieRouterNode>,
    fptr: Option<View>,
    varname: Option<~str>,
}

impl TrieRouterNode {
    pub fn new() -> TrieRouterNode {
        TrieRouterNode {
            children : SmallIntMap::new(),
            fptr: None,
            varname: None,
        }
    }

    fn add(&mut self, route: Route) {
        let path = route.path;

        if path[0] != '/' as u8 {
            // warn!("route path must begin with '/'");
        }
        if path.len() != 1 && path[path.len()-1] == '/' as u8 {
            // warn!("route path must not end with '/'");
        }

        {
            let mut ptr = self;
            for ch in path.chars() {
                let tmp = ptr;
                ptr = &mut tmp.children.find_or_insert(ch, &TrieRouterNode::new());
            }
            ptr.fptr = Some(route.fptr);
        }
    }

    fn add_variable(&mut self, route: Route) {
        let mut current_var = ~"";
        let mut building_var = false;
        let path = route.path;

        if path[0] != '/' as u8 {
            // error: route path must begin with '/'
        }
        if path.len() != 1 && path[path.len()-1] == '/' as u8 {
            // error: route path must not end with '/'
        }

        {
            let mut ptr = self;
            for ch in path.chars() {
                if building_var && ch == '/' {
                    if current_var.len() == 0 {
                        // error: can't allow empty string as varname
                    }
                    let tmp = ptr;
                    let tmp2 = &mut tmp.children.find_or_insert('^', &TrieRouterNode::new());
                    tmp2.varname = Some(current_var.clone());
                    ptr = &mut tmp2.children.find_or_insert(ch, &TrieRouterNode::new());
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
                    let tmp = ptr;
                    ptr = &mut tmp.children.find_or_insert(ch, &TrieRouterNode::new());
                }
            }
            if building_var {
                let tmp = ptr;
                let tmp2 = &mut tmp.children.find_or_insert('^', &TrieRouterNode::new());
                tmp2.varname = Some(current_var);
                ptr = tmp2;
            }
            ptr.fptr = Some(route.fptr);
        }
    }
}

impl Clone for TrieRouterNode {
    fn clone(&self) -> TrieRouterNode {
        let mut children: SmallIntMap<TrieRouterNode>;
        for (k,v) in self.children.iter() {
            children.insert(k,v.clone());
        }

        TrieRouterNode {
            children: children,
            fptr: self.fptr,
            varname: self.varname.clone()   
        }
    }
}

pub trait FindOrInsert {
    fn find_or_insert(&self, ch: char, node: &mut TrieRouterNode ) -> TrieRouterNode;
}

impl FindOrInsert for SmallIntMap<TrieRouterNode> {
    fn find_or_insert(&self, ch: char, node: &mut TrieRouterNode) -> TrieRouterNode {
        let mut result = node;
        if self.contains_key(&(ch as uint)) {
            result = self.find_mut(&(ch as uint)).unwrap();
        }
        else {
            self.insert(ch as uint, node.clone());
            result = self.find_mut(&(ch as uint)).unwrap();
        }
        result.clone()
    }
}