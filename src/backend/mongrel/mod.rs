extern crate zmq;

use oxidize::Oxidize;
use backend::OxidizeBackend;
use request;
use response;
use std::io::net::ip::SocketAddr;
use std::collections::HashMap;
use std::mem::transmute;
use std::cell::RefCell;

pub mod tnetstring;
mod connection;

/// A backend that uses Rust http to process the request and converts all the request data into the Oxidize structs
pub struct MongrelBackend {
    connection : RefCell<connection::Connection>,
    oxidize : Oxidize,
}

impl MongrelBackend {
    /// Creates a new Oxidize which contains the main guts of the web application
    pub fn new(oxidize: Oxidize) -> MongrelBackend {
        let mut ctx = zmq::Context::new();

        let mut connection = connection::connect(&mut ctx,
            Some("F0D32575-2ABB-4957-BC8B-12DAC8AFF13A".to_string()),
            vec!("tcp://127.0.0.1:9998".to_string()),
            vec!("tcp://127.0.0.1:9999".to_string()));
        MongrelBackend {
            connection: RefCell::new(connection),
            oxidize: oxidize,
        }
    }

    // fn mongrelRequestToOxidize(req: &connection::Request) -> request::Request {
        
    // }

    // fn oxidizeResponseToMongrel(res: &response::Response) -> () {

    // }
}

impl OxidizeBackend for MongrelBackend {
    /// As silly as it sounds, rust-http does not expose the `serve_forever` method,
    /// so this is a simple workaround to enable the framework user to start the server.
    fn serve(self) {
        let mut conn = self.connection.borrow_mut();
        loop {
            let request = conn.recv().unwrap();
            println!("uuid: {}", request.uuid);
            println!("id: {}", request.id);
            println!("path: {}", request.path);

            for (k, vs) in request.headers.iter() {
                for v in vs.iter() {
                    println!("header: {} => {}", *k, *v);
                }
            };
            println!("body: {}", request.body);
            // self.oxidize.handle_request(&mut my_request);
            conn.reply_http(&request,
                200u,
                "OK",
                connection::Headers(),
                "hello world!".to_string());
        }
    }
}