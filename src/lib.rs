//! Oxidize is a practical and extensible web framework written in rust. The main goal
//! of oxidize is to provide enough boiler plate without getting in the way
#![crate_id = "oxidize#0.2-pre"]

#![comment = "Rust Web Framework"]
#![license = "MIT/ASL2"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![doc(html_root_url = "http://www.rust-ci.org/jroweboy/oxidize/doc/")]

// TODO make sure that this is uncommented before committing
#![deny(missing_doc)]

#![feature(macro_rules)]
#![feature(phase)]
#[phase(syntax, link)] extern crate log;
// extern crate http;
extern crate time;
extern crate collections;
extern crate sync;
extern crate syntax;
extern crate test;
extern crate url;

pub use app::App;
pub use conf::Config;
pub use common::{status, method};
pub use request::Request;
pub use middleware::MiddleWare;
pub use response::Response;
pub use oxidize::Oxidize;

pub mod oxidize;
pub mod app;
pub mod router;
pub mod request;
pub mod conf;
pub mod middleware;
pub mod common;
pub mod response;
pub mod backend;

// TODO move the macro to a seperate crate as per https://github.com/Ogeon/rustful/issues/1

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