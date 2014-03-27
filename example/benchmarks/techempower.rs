extern crate oxidize;
extern crate collections;
extern crate serialize;
extern crate postgres;
extern crate extra;

use oxidize::{Oxidize, Request, ResponseWriter, Config, MediaType};
use oxidize::route::Router;
use oxidize::route::regexrouter::{RegexRouter, RegexRoute, Route, Simple};
use std::cast;
use std::io::net::ip::SocketAddr;
use std::rand::{task_rng, Rng};
use serialize::{json, Encodable};

use postgres::{PostgresConnection, PostgresStatement, NoSsl};
use postgres::pool::PostgresConnectionPool;
use postgres::types::ToSql;

// Database
static connectionString:    &'static str = "postgres://benchmarkdbuser:benchmarkdbpass@localhost/hello_world";
static worldSelect:         &'static str = "SELECT id, randomNumber FROM World WHERE id = $1";
static worldUpdate:         &'static str = "UPDATE World SET randomNumber = ? WHERE id = ?";
static fortuneSelect:       &'static str = "SELECT id, message FROM Fortune;";
static worldRowCount:       i64 = 10000;
static maxConnectionCount:  int = 256;
static connectionPoolCount: uint = 5;

static mut connectionPoolOption: Option<*PostgresConnectionPool> = None;

fn main() {
    let routes: ~[RegexRoute] = ~[
        Simple(Route{ method: "GET", name: "json",      path: "/json",      fptr: json_handler}),
        Simple(Route{ method: "GET", name: "db",        path: "/db",        fptr: single_query_handler}),
        Simple(Route{ method: "GET", name: "queries",   path: "/queries",   fptr: multiple_queries_handler}),
        Simple(Route{ method: "GET", name: "fortunes",  path: "/fortunes",  fptr: fortunes_handler}),
        Simple(Route{ method: "GET", name: "updates",   path: "/updates",   fptr: updates_handler}),
        Simple(Route{ method: "GET", name: "plaintext", path: "/plaintext", fptr: plaintext_handler}),
    ];

    let router = ~RegexRouter::new(routes);
    let conf = Config {
        debug: true,
        bind_addr: from_str::<SocketAddr>("127.0.0.1:8001").unwrap(),
    };

    //Database Connection Pool
    let pool = PostgresConnectionPool::new(connectionString, NoSsl, connectionPoolCount);
    unsafe {
        connectionPoolOption = Some(cast::transmute(&pool));
    }

    let server = Oxidize::new(conf, router as ~Router);
    server.serve();
}

#[deriving(Encodable)]
pub struct JSONWorld {
    id: i64,
    random_number: i64
}

#[deriving(Encodable)]
pub struct JSONFortune {
    id: &'static int,
    message: &'static str
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
        MediaType {type_: ~"application",subtype: ~"json",parameters: ~[]}, 
        json::Encoder::str_encode(&message)
    );
}

#[allow(unused_must_use)]
fn single_query_handler(request: &Request, response: &mut ResponseWriter) {
    unsafe {
        match connectionPoolOption {
            Some(connectionPoolVoidPointer) => {
                let connectionPool: *PostgresConnectionPool = cast::transmute(connectionPoolVoidPointer);
                let conn = (*connectionPool).get_connection();
                let stmt = conn.prepare(worldSelect);
                let mut rng = task_rng();
                let row: i64 = rng.gen_range(1i64, worldRowCount + 1);

                for row in stmt.query([&row as &ToSql]) {
                    let world = JSONWorld {
                        id: row[1],
                        random_number: row[2],
                    };
                    
                    response.write_content_auto(
                        MediaType {type_: ~"application",subtype: ~"json",parameters: ~[]}, 
                        json::Encoder::str_encode(&world)
                    );
                }
            }
            None => { println!("Could not get database connection pool");}
        }
    }
}

#[allow(unused_must_use)]
fn multiple_queries_handler(request: &Request, response: &mut ResponseWriter) {

}

#[allow(unused_must_use)]
fn fortunes_handler(request: &Request, response: &mut ResponseWriter) {

}

#[allow(unused_must_use)]
fn updates_handler(request: &Request, response: &mut ResponseWriter) {

}


#[allow(unused_must_use)]
fn plaintext_handler(request: &Request, response: &mut ResponseWriter) {
    response.write_content_auto(
        MediaType {type_: ~"text",subtype: ~"plain",parameters: ~[]}, 
        ~"Hello, World!"
    );
}