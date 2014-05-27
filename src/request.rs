//! A Request is a struct that is passed through the middleware (TODO on that part) 
//! and will contain data about the HTTP Request. It will be passed through the 
//! middleware as mutable so as to allow them to change things about the request, 
//! but there probably isn't a good reason to change the request in your controller method

use http::method::Method;
use collections::hashmap::HashMap;
use http::server::request::{AbsolutePath, AbsoluteUri, Authority, Star};
use url::Url;

#[allow(uppercase_variables)]
/// One of the goals of the Request struct is to contain any and all relevent information 
/// in an easy to remember location while allowing the middleware to edit 
pub struct Request {
    /// CONNECT POST GET etc as defined by rust-http
    pub method : Method,
    /// The request URL as a string. TODO: should this be a url::Url instead?
    pub uri: StrBuf,
    /// A HashMap of query params in Key Value pairing. 
    /// ex: ?foo=bar&baz=qux the hashmap now has foo => bar, baz => qux
    pub GET : Option<HashMap<StrBuf, StrBuf>>,
    /// Currently None, but meant to provide the same details that GET provides
    pub POST : Option<HashMap<StrBuf, StrBuf>>,
    /// Not in use currently, but I plan on changing this to allow an authentication 
    /// middleware to attach a struct to represent a User
    pub user : Option<StrBuf>,
}

// fn parse_vars(vars: &str) -> HashMap<~str, ~str> {
//     let mut map = HashMap::new();
//     for var in vars.split('&') {
//         let a : Vec<&str> = var.split('=').collect();
//         map.insert(a.get(0).to_owned(), a.get(1).to_owned());
//     }
//     return map;
// }

impl Request {
    /// Creates the oxidize specific Request struct from the underlying http libraries request.
    /// The goal is to provide a request struct that provides convenient access to common things
    /// like get and post, while still retaining all the gory HTTP details.
    pub fn new(req: &::http::server::Request) -> Request {
        let path = match req.request_uri {
            AbsolutePath(ref i) => from_str::<Url>(i.to_str()).unwrap(),
            AbsoluteUri(ref i) => i.clone(),
            Authority(ref i) => from_str::<Url>(i.to_str()).unwrap(),
            Star => fail!("Star option is not supported yet")
        };
        // Add get params to the request
        let mut option_get = None;
        if path.query.len() > 0 {
            let mut get = HashMap::new();
            for q in path.query.iter() {
                let (ref a, ref b) = *q;
                get.insert(a.clone(), b.clone());
            }
            option_get = Some(get);
        }

        // TODO add post params
        Request {
            method: req.method.clone(),
            uri: path.to_str().to_strbuf(),
            GET: option_get,
            POST: None,
            user: None
        }
    }
}

/* What follows is most of Go's net/http module's definition of Request.
Its got some really good ideas of what my struct should end up looking like (That or maybe teepee will make it this way)

pub struct Request {
    // GET, POST, etc.
    method: ~Method,

    // The URL requested, constructed from the request line and (if available)
    // the Host header.
    url: ~Url,

    // The HTTP protocol version used; typically (1, 1)
    protocol: (uint, uint),

    // Request headers, all nicely and correctly parsed.
    headers: ~Headers,

    // The message body.
    body: Reader,

    // ContentLength records the length of the associated content.
    // The value -1 indicates that the length is unknown.
    // Values >= 0 indicate that the given number of bytes may
    // be read from Body.
    // For outgoing requests, a value of 0 means unknown if Body is not nil.
    content_length: i64,

    // TransferEncoding lists the transfer encodings from outermost to
    // innermost. An empty list denotes the "identity" encoding.
    // TransferEncoding can usually be ignored; chunked encoding is
    // automatically added and removed as necessary when sending and
    // receiving requests.
    transfer_encoding: ~[~str],

    // Close indicates whether to close the connection after
    // replying to this request.
    close: bool,

    // The host on which the URL is sought.
    // Per RFC 2616, this is either the value of the Host: header
    // or the host name given in the URL itself.
    // It may be of the form "host:port".
    host: ~str,

    // Form contains the parsed form data, including both the URL
    // field's query parameters and the POST or PUT form data.
    // This field is only available after ParseForm is called.
    // The HTTP client ignores Form and uses Body instead.
    form: url.Values,

    // PostForm contains the parsed form data from POST or PUT
    // body parameters.
    // This field is only available after ParseForm is called.
    // The HTTP client ignores PostForm and uses Body instead.
    post_form: url.Values,

    // MultipartForm is the parsed multipart form, including file uploads.
    // This field is only available after ParseMultipartForm is called.
    // The HTTP client ignores MultipartForm and uses Body instead.
    multipart_form: *multipart.Form,

    // Trailer maps trailer keys to values.  Like for Header, if the
    // response has multiple trailer lines with the same key, they will be
    // concatenated, delimited by commas.
    // For server requests, Trailer is only populated after Body has been
    // closed or fully consumed.
    // Trailer support is only partially complete.
    trailer: ~Headers,

    // RemoteAddr allows HTTP servers and other software to record
    // the network address that sent the request, usually for
    // logging. This field is not filled in by ReadRequest and
    // has no defined format. The HTTP server in this package
    // sets RemoteAddr to an "IP:port" address before invoking a
    // handler.
    // This field is ignored by the HTTP client.
    remote_addr: string,

    // RequestURI is the unmodified Request-URI of the
    // Request-Line (RFC 2616, Section 5.1) as sent by the client
    // to a server. Usually the URL field should be used instead.
    // It is an error to set this field in an HTTP client request.
    request_uri: string,

    // TLS allows HTTP servers and other software to record
    // information about the TLS connection on which the request
    // was received. This field is not filled in by ReadRequest.
    // The HTTP server in this package sets the field for
    // TLS-enabled connections before invoking a handler;
    // otherwise it leaves the field nil.
    // This field is ignored by the HTTP client.
    tls: *tls.ConnectionState,
}*/