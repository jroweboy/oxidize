//! App is the trait that one must implement in order to add this to the oxidize struct
//! The general idea is the router will find which handler it needs to call and then 
//! it will pass a string of the handler to `handle_route` where `handle_route` will
//! will be able to match on that string. The eventual goal is to write a macro that will
//! generate both of the required functions here so that the framework user will not 
//! have to worry about the specific details. (TODO)

use request::Request;
use http::server::ResponseWriter;
use route::{Router, RouteInfo};

pub trait App:Send+Share {
    /// receive a string that indicates which route to call and also any of the
    /// parsed url parameters. This function should bind these parameters 
    /// to variables and pass them to the requested user function (TODO)
    fn handle_route(&self, info: RouteInfo, &mut Request, &mut ResponseWriter);

    /// a static function that will create a list of the routes and prepare a 
    /// router for the application to use. By handling the routes in this manner,
    /// it should be fairly simple to one day support pluggable applications
    fn get_router() -> Box<Router:Send+Share>;
}