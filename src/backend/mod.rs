//! A module that contains built in backends for oxidize. Currently I have only plans to support just 
//! Mongrel and Rust Http until teepee comes out at which point I'll finally drop support for rust-http
//! and add in teepee support. If someone makes a new backend they wish to add to the repo, just send a pull request
//! More information about backends can be found by reading the documentation for the trait and by reading
//! the example backends found in the source.

/// An OxidizeBackend is anything that performs the actual handling of the HTTP Request 
/// and serving of the HTTP Response. This could be through any method such as rust-http, Mongrel.
/// or an Apache module, but no matter the backend, it will need to expose these methods. In addition,
/// the backend upon receiving a Request, should call the method handle_request on oxidize which will
/// handle all the routing details and forwards correct requests to the framework user.
pub trait OxidizeBackend {
    // /// Return a new copy of your Backend
    // fn new(Oxidize) -> Self;
    /// This method will be called by the oxidize struct and will be expected to wait for a request to the server.
    fn serve(self);
    // /// After the Response has been completed, your backend will need to send it back to the user.
    // /// This step will be called by the oxidize struct
    // fn receive_response(&self, Response);
}

#[allow(missing_doc)]
pub mod mongrel;
pub mod rusthttp;