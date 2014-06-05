//! Response is a generic container which must be returned by the framework user and it will contain
//! all the data that will eventually become the HTTP Response. The actual conversion of Response to 
//! HTTP Response is the responsibility of the specific backend

use common::status;

#[deriving(Clone, PartialEq, Eq)]
/// A small wrapper of the MediaType. This is expected to change when teepee is released and then 
/// I will probably have teepee as a required dependancy and just use their types.
#[allow(missing_doc)]
pub struct MediaType {
    pub type_: String,
    pub subtype: String,
    pub parameters: Vec<(String, String)>,
}

/// TODO: Very lack luster at the moment but yeah. I'm not sure what I want in here yet anyway
pub struct Response {
    /// The body of the Response
    pub content: String,
    /// HTTP Status code. If you don't want to look up the appropriate status, use one of the 
    /// default constructors for a specific status (eg fn ok)
    pub status: status::Status,
    #[allow(missing_doc)]
    pub content_type: MediaType,
}

impl Response {
    /// A simple method to return a 200 OK with no response body
    pub fn empty() -> Response {
        Response {
            content: "".to_string(),
            status: status::Ok,
            content_type: MediaType {
                type_: "text".to_string(),
                subtype: "plain".to_string(),
                parameters: Vec::new(),
            },
        }
    }
    /// Create a new response with the given status and content
    pub fn new(status: status::Status, body: String, content_type: Option<MediaType>) -> Response {
        Response {
            content: body,
            status: status,
            content_type: content_type.unwrap_or(
                MediaType {
                    type_: "text".to_string(),
                    subtype: "plain".to_string(),
                    parameters: Vec::new(),
                }
            ),
        }
    }

    // TODO: Maybe generate a bunch of these with a macro somehow?
    /// Create a new response with the status 200 Ok
    pub fn ok(body: String, content_type: Option<MediaType>) -> Response {
        Response {
            content: body,
            status: status::Ok,
            content_type: content_type.unwrap_or(
                MediaType {
                    type_: "text".to_string(),
                    subtype: "html".to_string(),
                    parameters: Vec::new(),
                }
            ),
        }
    }

    /// Create a new response with the status 404 Not Found
    pub fn not_found(body: String, content_type: Option<MediaType>) -> Response {
        Response {
            content: body,
            status: status::NotFound,
            content_type: content_type.unwrap_or(
                MediaType {
                    type_: "text".to_string(),
                    subtype: "html".to_string(),
                    parameters: Vec::new(),
                }
            ),
        }
    }
    /// Create a new response with the status 404 Not Found
    pub fn bad_request(body: String, content_type: Option<MediaType>) -> Response {
        Response {
            content: body,
            status: status::BadRequest,
            content_type: content_type.unwrap_or(
                MediaType {
                    type_: "text".to_string(),
                    subtype: "html".to_string(),
                    parameters: Vec::new(),
                }
            ),
        }
    }
}