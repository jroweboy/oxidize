extern mod http;
use http::server::{Config, Server, Request, ResponseWriter};
use http::headers;
use std::io::net::ip::{SocketAddr, Ipv4Addr};

use router::Router;
use renderer::Renderer;

mod router;
mod renderer;

#[deriving(Clone)]
struct OxidizeServer;

struct OxidizeRouter;

struct OxidizeRenderer;

impl Renderer for OxidizeRenderer {
  fn render(&self) -> ~str {
    return ~"\
      <html><body><h1>It works!</h1>\n\
      <p>This is the default web page for this server.</p>\n\
      <p>The web server software is running but no content has been added, yet.</p>\n\
      </body></html>\n";
  }
}

impl Router for OxidizeRouter {
  fn route(&self) -> ~str {
    return OxidizeRenderer.render();
  }
}

impl Server for OxidizeServer {
  fn get_config(&self) -> Config {
    Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
  }

  fn handle_request(&self, _r: &Request, w: &mut ResponseWriter) {
    let response = OxidizeRouter.route();

    w.headers.content_type = Some(headers::content_type::MediaType {
      type_: ~"text",
      subtype: ~"html",
      parameters: ~[]
    });

    w.headers.content_length = Some(response.len());

    w.write(response.as_bytes());
  }
}

fn main(){
  OxidizeServer.serve_forever();
  OxidizeRouter.route();
}