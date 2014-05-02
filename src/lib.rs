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
// #![feature(macro_rules)]
// #![macro_escape]
#![feature(phase)]
#[phase(syntax, link)] extern crate log;
extern crate http;
extern crate time;
extern crate collections;
extern crate sync;

pub mod oxidize;
pub mod app;
pub mod route;
pub mod request;
pub mod conf;