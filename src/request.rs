//! A Request is a struct that is passed through the middleware (TODO on that part) 
//! and will contain data about the HTTP Request. It will be passed through the 
//! middleware as mutable so as to allow them to change things about the request, 
//! but there probably isn't a good reason to change the request in your controller method

use http::method::Method;
use collections::hashmap::HashMap;

#[allow(uppercase_variables, missing_doc)]
pub struct Request<'a> {
    pub method : Method,
    pub uri: ~str,
    pub GET : Option<HashMap<~str, ~str>>,
    pub POST : Option<HashMap<~str, ~str>>,
    /// Not in use currently, but I plan on changing this to allow an authentication 
    /// middleware to attach a struct to represent a User
    pub user : Option<~str>,
}