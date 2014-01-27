extern mod http;

use response::Response;
use http::server::Request;

mod response;

type Route<'a> = (&'a str, &'a str, fn(&Request) -> ~Response);