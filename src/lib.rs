//! oxidize is a practical and extensible web framework written in rust. The main goal
#![crate_id = "oxidize#0.2-pre"]

#![comment = "Rust Web Framework"]
#![license = "MIT/ASL2"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![doc(html_root_url = "http://www.rust-ci.org/jroweboy/oxidize/doc/")]

// TODO make sure that this is uncommented before committing
// #![deny(missing_doc)]

#![feature(default_type_params)]
#![feature(macro_rules)]
// #![macro_escape]
#![feature(phase)]
#[phase(syntax, link)] extern crate log;
extern crate http;
extern crate time;
extern crate collections;
extern crate sync;


pub use app::App;
pub use conf::Config;
pub use http::status;
pub use request::Request;
pub use middleware::MiddleWare;
pub use http::server::ResponseWriter;
pub use http::headers::content_type::MediaType;
pub use oxidize::Oxidize;

pub mod oxidize;
pub mod app;
pub mod route;
pub mod request;
pub mod conf;
pub mod middleware;

// pub use app::App;
// pub use config::Config;
// pub use oxidize::Oxidize;
// pub use request::Request;
// pub use http::server::ResponseWriter;
// pub use http::headers::content_type::MediaType;


// --- HERE BE MACROS ---

/**
The routes macro provides a convenient way to define the routes and implementing 
the required traits for oxidize. It will also automatically attempt to bind any
variables passed by the url and call the method with the name provided.

```
routes!(AppName Trierouter,
    ("GET", "/", "index", self.index),
    ("GET", "/test", "test_mustache", self.test_mustache),
    ("GET", "/users/user-<userid:uint>/post-<postid:uint>", "test_variable", self.test_variable),
)
```
*/

// Ugh, it looks like I'll need to change to the more complicated macro_registar instead
// #[macro_export]
// macro_rules! routes(
//     ($app_name:ident, $router_name:ident, $(($method:expr, $path:expr, $name:expr, $func:expr)),+) => (
//     impl App for $app_name {
//         fn handle_route(&self, info: RouteInfo, req: &mut Request, res: &mut ResponseWriter){
//             match (info.name, info.method) {
//                 $(($name, $method) => {$func(req, res)}, )+ 
//             }
//         }

//         fn get_router() -> ~Router:Send+Share {
//             let router = ~$router_name::new();
//             let routes = vec!( $(($method, $path, $name)),+);
//             for (m, p, n) in routes {
//                 router.add_route(m, p, n);
//             }
//             router as ~Router:Send+Share
//         }
//     }
//     )
// )

/**
A macro for assigning content types. This macro was written by Ogeon for rustful 
https://github.com/Ogeon/rustful but it is extremely useful and relevent so I had to 
add it to oxidize as well.

It takes a main type, a sub type and a parameter list. Instead of this:

```
response.headers.content_type = Some(MediaType {
type_: StrBuf::from_str("text"),
subtype: StrBuf::from_str("html"),
parameters: vec!((StrBuf::from_str("charset"), StrBuf::from_str("UTF-8")))
});
```

it can be written like this:

```
response.headers.content_type = content_type!("text", "html", "charset": "UTF-8");
```

The `"charset": "UTF-8"` part defines the parameter list for the content type.
It may contain more than one parameter, or be omitted:

```
response.headers.content_type = content_type!("application", "octet-stream", "type": "image/gif", "padding": "4");
```

```
response.headers.content_type = content_type!("image", "png");
```
**/
#[macro_export]
macro_rules! content_type(
    ($main_type:expr, $sub_type:expr) => ({
        Some(::http::headers::content_type::MediaType {
            type_: StrBuf::from_str($main_type),
            subtype: StrBuf::from_str($sub_type),
            parameters: Vec::new()
        })
    });

    ($main_type:expr, $sub_type:expr, $($param:expr: $value:expr),+) => ({
        Some(::http::headers::content_type::MediaType {
            type_: StrBuf::from_str($main_type),
            subtype: StrBuf::from_str($sub_type),
            parameters: vec!( $( (StrBuf::from_str($param), StrBuf::from_str($value)) ),+ )
        })
    });
)