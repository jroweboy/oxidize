//! App is the trait that one must implement in order to add this to the oxidize struct
//! The general idea is the router will find which handler it needs to call and then 
//! it will pass a string of the handler to `handle_route` where `handle_route` will
//! will be able to match on that string. The eventual goal is to write a macro that will
//! generate both of the required functions here so that the framework user will not 
//! have to worry about the specific details. (TODO)

use request::Request;
use http::server::ResponseWriter;
use http::status;
use collections::hashmap::HashMap;
use router::Router;

/// In order for your application to receive the request and response data from Oxidize
/// it needs to fulfill the following trait. This defines a simple method for enabling
/// modular applications (at least I hope) and also communication to/from oxidize
pub trait App:Send+Share {
    /// Receive a `&&static str` that indicates which route to call and also any of the
    /// parsed url parameters. This function should bind these parameters 
    /// to variables and pass them to the requested user function (TODO make the macro for this)
    fn handle_route(&self, Option<&&'static str>, Option<HashMap<~str,~str>>, &mut Request, &mut ResponseWriter);

    /// Create a list of the routes and prepare a 
    /// router for the application to use. By handling the routes in this manner,
    /// it should be fairly simple to one day support pluggable applications 
    fn get_router(&self) -> Router<&'static str>;

    /// The user can override this method with a custom 404 function
    /// ... at least that is the idea. I have no clue if it will work in practice
    #[allow(unused_variable,unused_must_use)]
    fn default_404(&self, req: &mut Request, res: &mut ResponseWriter) {
        res.status = status::NotFound;
        res.write_content_auto(
            ::http::headers::content_type::MediaType {
                type_: StrBuf::from_str("text"),
                subtype: StrBuf::from_str("html"),
                parameters: Vec::new()
            }, 
            StrBuf::from_str("<h1>404 - Not Found</h1>")
        ); 
    }
}