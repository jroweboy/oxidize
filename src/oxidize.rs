extern mod http;
use http::server::{Config, Server, Request, ResponseWriter};
use http::headers;
use std::io::net::ip::{SocketAddr, Ipv4Addr};

use router::Router;

mod router;

#[deriving(Clone)]
struct Oxidize;

impl Server for Oxidize {
  fn get_config(&self) -> Config {
    Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
  }

  fn handle_request(&self, _r: &Request, w: &mut ResponseWriter) {
    let response = ~"\
      <html><body><h1>It works!</h1>\n\
      <p>This is the default web page for this server.</p>\n\
      <p>The web server software is running but no content has been added, yet.</p>\n\
      </body></html>\n";

    w.headers.content_type = Some(headers::content_type::MediaType {
      type_: ~"text",
      subtype: ~"html",
      parameters: ~[]
    });

    w.headers.content_length = Some(response.len());

    w.write(response.as_bytes());
  }
}

impl Router for Oxidize {
  fn something() -> () {
    
  }
}

fn main(){
  Oxidize.serve_forever();
}