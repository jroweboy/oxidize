//! These are two common classes that were needed from rust-http. Instead of having a hard dependancy on
//! rust-http (which is slowly going away) I decided instead to move the useful mods into my repo for now
//! and eventually these can be replaced with other versions if it is found benificial. In particular, I hope
//! that I can replace this with httpcommon from teepee once that is finished.
#![allow(missing_doc)]

pub mod method;
pub mod status;