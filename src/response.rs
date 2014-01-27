extern mod http;

use http::status::Status;
use http::status;

pub struct Response {
  content : ~str,
  status: Status
}

impl Response {
  pub fn new(status : Status, content : ~str) -> Response {
    Response {content: content, status: status}
  }
}