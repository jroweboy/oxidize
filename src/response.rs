extern mod http;

use http::status::Status;

// Very lack luster at the moment but yeah. I'm not sure what I want in here yet anyway
pub struct Response {
  content : ~str,
  status: Status
}
