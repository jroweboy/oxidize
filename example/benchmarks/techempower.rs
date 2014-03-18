extern crate oxidize;
extern crate collections;
extern crate serialize;
extern crate postgres;
extern crate extra;

use oxidize::{Oxidize, Request, ResponseWriter, Config, MediaType};
use oxidize::route::Router;
use oxidize::route::regexrouter::{RegexRouter, RegexRoute, Route, Simple};
use std::io::net::ip::SocketAddr;
use serialize::{json, Encodable};

use postgres::{PostgresConnection, PostgresStatement, NoSsl};
use postgres::types::ToSql;



fn main() {
    let routes: ~[RegexRoute] = ~[
        Simple(Route{ method: "GET", name: "json", path: "/json", fptr: json_handler}),
        Simple(Route{ method: "GET", name: "plaintext", path: "/plaintext", fptr: plaintext_handler}),
    ];

    let router = ~RegexRouter::new(routes);
    let conf = Config {
        debug: true,
        bind_addr: from_str::<SocketAddr>("127.0.0.1:8001").unwrap(),
    };

    //Database Connection Example
    //let conn = PostgresConnection::connect("postgres://postgres@localhost", &NoSsl);

    let server = Oxidize::new(conf, router as ~Router);
    server.serve();
}


#[deriving(Encodable)]
pub struct JSONMessage   {
    message: &'static str,
}

static message : JSONMessage = JSONMessage{message:"Hello, World!"};

//These would be nice in the future but we need to update rust-http to take a lifetime instead of an owned string
//static message_type_app_js : MediaType = MediaType {type_: "application",subtype: "javascript",parameters: ~[]};
//static message_type_plain  : MediaType = MediaType {type_: "text",subtype: "plain",parameters: ~[]};

#[allow(unused_must_use)]
fn json_handler(request: &Request, response: &mut ResponseWriter) {
    response.write_content_auto(
        MediaType {type_: ~"application",subtype: ~"javascript",parameters: ~[]}, 
        json::Encoder::str_encode(&message)
    );
}

#[allow(unused_must_use)]
fn plaintext_handler(request: &Request, response: &mut ResponseWriter) {
    response.write_content_auto(
        MediaType {type_: ~"text",subtype: ~"plain",parameters: ~[]}, 
        ~"Hello, World!"
    );
}