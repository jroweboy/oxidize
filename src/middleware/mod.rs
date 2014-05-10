//! Middleware are special structs that have the ability to intercept and modify both the 
//! request and response before and after the framework user's methods are called.
//! Middleware is typically used for implementing things such as a built in authentication system
//! and other things such as logging and profilers. 

use request::Request;
use http::server::ResponseWriter;
use std::vec::Vec;

pub trait MiddleWare:Send+Share {
    fn apply(&self, &mut Request, &mut ResponseWriter);
    fn copy(&self) -> Box<MiddleWare:Send+Share>;
}

impl Clone for Box<MiddleWare:Send+Share> {
    fn clone(&self) -> Box<MiddleWare:Send+Share> {
        self.copy()
    }
}