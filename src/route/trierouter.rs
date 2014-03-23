use collections::hashmap::HashMap;
use http::headers::content_type::MediaType;
use std::io::File;
use request;
use http::status;
use sync::Arc;
use route::Router;
use request::Request;
use http::server::ResponseWriter;

//TODO: ensure routes are valid URLs

pub type View = fn (&Request, &mut ResponseWriter, &~[(~str,~str)]);

pub struct TrieRouter<'a> {
    trie: Arc<OxidizeTrie>,
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

    fn reverse<'a>(&'a self, name: &str, vars: Option<~[(~str,~str)]>) -> Option<&'a ~str> {
        // TODO: use the vars to replace 
        self.reverse_routes.get().find_equiv(&name).and_then(
            |path: &'a ~str| { Some(path) }
        )
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
        let trie = OxidizeTrie::new(&routes);
        TrieRouter {
            trie: Arc::new(trie),
            reverse_routes: Arc::new(reverse_routes)
        }
	}

	pub fn get_handler(&'a self, uri: &str) -> (Option<View>,~[(~str,~str)]) {
		let mut path = uri.to_owned();
		let trie = &self.trie;

		if(path.len() != 1 && path[path.len() - 1] == '/' as u8) {
			path.pop_char();
		}

		let mut current_node = &trie.get().root;
		let mut current_var = ~"";
		let mut current_key = ~"";
		let mut building_var = false;
		let mut not_found = false;
		let mut vars = ~[];

		for ch in path.chars() {
			if (building_var && ch == '/') {
				vars.push((current_key,current_var));
				building_var = false;
				if(current_node.children.contains_key(&ch)) {
					current_node = current_node.children.get(&ch);
				}
				else {
					not_found = true;
					break;
				}
			}
			else if (building_var) {
				current_var.push_char(ch);
			}
			else if(current_node.children.contains_key(&'*')) {
				current_node = current_node.children.get(&'*');
				current_key = match current_node.varname{
					Some(k) => k,
					None => ~""
				};
				current_var.truncate(0);
				current_var.push_char(ch);
				building_var = true;
			}
			else if(current_node.children.contains_key(&ch)) {
				current_node = current_node.children.get(&ch);
			}
			else {
				not_found = true;
				break;
			}
		}
		(current_node.fptr, vars)
	}
}

pub enum TrieRoute {
    Simple(Route),
    Variable(Route)
}

pub struct OxidizeTrie {
	root: OxidizeTrieNode,
}

impl OxidizeTrie {
	pub fn new(routes : &~[TrieRoute]) -> OxidizeTrie {
		let mut head = OxidizeTrieNode::new();

		for route in routes.iter() {
			match *route {
				Simple(r) => head.add(r),
				Variable(r) => head.add_variable(r)
			}
		}

		OxidizeTrie {
			root: head,
		}
	}
}

pub struct OxidizeTrieNode {
	fptr: Option<View>,
	varname: Option<~str>,
	children: HashMap<char,OxidizeTrieNode>
}

impl OxidizeTrieNode {
	pub fn new() -> OxidizeTrieNode {
		OxidizeTrieNode {
			fptr: None,
			varname: None,
			children: HashMap::<char,OxidizeTrieNode>::new()
		}
	}

	pub fn add<'a>(&'a mut self, route: Route) {
		let mut current_node = self;
		let mut current_char: char;
		let mut current_var: ~str;
		let mut building_var = false;
		let path = route.path;

		if(path[0] != '/' as u8) {
			// error: route path must begin with '/'
		}
		if(path.len() != 1 && path[path.len()-1] == '/' as u8) {
			// error: route path must not end with '/'
		}

		for ch in path.chars() {
			current_node = current_node.children.find_or_insert(ch,OxidizeTrieNode::new());
		}
		current_node.fptr = Some(route.fptr);
	}

	pub fn add_variable<'a>(&'a mut self, route: Route) {
		let mut current_node = self;
		let mut current_char: char;
		let mut current_var = ~"";
		let mut building_var = false;
		let path = route.path;

		if(path[0] != '/' as u8) {
			// error: route path must begin with '/'
		}
		if(path.len() != 1 && path[path.len()-1] == '/' as u8) {
			// error: route path must not end with '/'
		}

		for ch in path.chars() {
			if (building_var && ch == '/') {
				if(current_var.len() == 0) {
					// error: can't allow empty string as varname
				}
				current_node.varname = Some(current_var.clone());
				current_node = current_node.children.find_or_insert('*',OxidizeTrieNode::new());
				current_node = current_node.children.find_or_insert(ch,OxidizeTrieNode::new());
				building_var = false;
			}
			else if (building_var) {
				current_var.push_char(ch);
			}
			else if(ch == ':') {
				building_var = true;
				current_var.truncate(0);
			}
			else {
				current_node = current_node.children.find_or_insert(ch,OxidizeTrieNode::new());
			}
		}
		current_node.fptr = Some(route.fptr);
		if(building_var) {
			current_node.varname = Some(current_var);
		}
	}
}

//TODO: seems like this shouldn't be in declared in multiple files
pub struct Route {
    method: &'static str,
    path : &'static str,
    name : &'static str,
    fptr : View,
}