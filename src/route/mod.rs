//! A router is any struct that is able to parse a given url, and return the RouteInfo
//! needed by the handle_route function on App
use std::vec::Vec;

pub trait Router:Send+Share {
    /// add_route is meant to be called in the get_router function
    /// I chose to make the strings static since it simplifies the most common use case
    /// where you define all the routes in a list of tuples containing the method etc
    /// (using the macro that I will write at some point)
    fn new() -> Self;
    fn add_route(&self, method: &'static str, url: &'static str, name: &'static str);
    fn route<'a>(&self, method: &'a str, url: &'a str) -> RouteInfo<'a>;
    fn reverse<'a>(&'a self, name: &str, vars: Option<Vec<(~str,~str)>>) -> Option<~str>;
    fn copy(&self) -> Box<Router:Send+Share>;
}

impl Clone for Box<Router:Send+Share> {
    fn clone(&self) -> Box<Router:Send+Share> {
        self.copy()
    }
}

pub struct RouteInfo<'a> {
    pub name: &'a str,
    pub method: &'a str,
    pub params: Option<Vec<(~str,~str)>>,
}

// pub mod trierouter;
pub mod matchrouter;