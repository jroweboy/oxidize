extern mod http;
extern mod extra;

use http::server::{Config, Server, Request, ResponseWriter};
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};
use http::headers;
use http::status;
use http::status::Status;
use http::method::Method;

use extra::time;
use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::File;
use std::str;
use std::hashmap::HashMap;

use router::Router;
use renderer::Renderer;
use response::Response;

mod router;
mod renderer;
mod response;

#[deriving(Clone)]
struct OxidizeServer;

struct OxidizeRouter;

struct OxidizeRenderer;

impl Renderer for OxidizeRenderer {
  fn render(&self, request: &Request, file_name: &str, context : Option<HashMap<&str, &str>>) -> ~str {
    let contents = File::open(&Path::new("views/"+file_name+".html")).read_to_end();
    return str::from_utf8(contents).to_owned();
  }
}

impl Router for OxidizeRouter {
  // should probably return a result object
  // containing the body and the status
  fn route(&self, request: &Request, response: &mut ResponseWriter) -> ~str {
    let path = match request.request_uri{
      AbsolutePath(ref i) => i.to_str(),
      AbsoluteUri(ref i) => i.to_str(),
      Authority(ref i) => i.to_str(),
      Star => ~"error" // ?
    };

    let method = request.method.to_str();

    let res = match (method,path){
      (~"GET",~"/") => index(request),
      (~"GET",~"/test") => test(request),
      (x,y) => notFound(request)
    };

    let reason = res.status.reason();
    let code = res.status.code();

    let newStatus = Status::from_code_and_reason(code,reason);

    response.status = newStatus;
    return res.content;
  }
}

impl Server for OxidizeServer {
  fn get_config(&self) -> Config {
    Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
  }

  fn handle_request(&self, req: &Request, res: &mut ResponseWriter) {
    res.headers.date = Some(time::now_utc());
    res.headers.server = Some(~"Oxidize/0.0.0 (Ubuntu)");

    let response_body = OxidizeRouter.route(req,res);

    res.headers.content_type = Some(headers::content_type::MediaType {
      type_: ~"text",
      subtype: ~"html",
      parameters: ~[]
    });

    res.headers.content_length = Some(response_body.len());

    res.write(response_body.as_bytes());
  }
}

pub fn notFound(request: &http::server::Request) -> Response {
  return Response::new(status::NotFound, OxidizeRenderer.render(request, "404", None));
}

pub fn index(request: &http::server::Request) -> Response {
  return Response::new(status::NotFound, OxidizeRenderer.render(request, "index", None));
}

pub fn test(request: &http::server::Request) -> Response {
  return Response::new(status::NotFound, OxidizeRenderer.render(request, "test", None));
}

fn main(){
  OxidizeServer.serve_forever();
}