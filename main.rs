
#[crate_id = "oxidize"];

#[desc = "A micro web framework for Rust"];
#[license = "MIT"];

extern mod extra;
extern mod http;
//extern mod http = "github.com/chris-morgan/rust-http";

//mod routes;

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::Writer;
use std::io::File;
use extra::time;
use extra::json::{Json, Decoder, ToJson, Object};
use extra::list::List;
use extra::treemap::TreeMap;
use std::str;

use http::server::{Config, Server, Request, ResponseWriter};
use http::headers;

use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};

#[deriving(Clone)]
struct OxidizeServer;

impl Server for OxidizeServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
    }

    fn handle_request(&self, _r: &Request, w: &mut ResponseWriter) {
        w.headers.date = Some(time::now_utc());
        w.headers.server = Some(~"Oxidize/0.0.0 (Ubuntu)");

        /*w.headers.last_modified = Some(time::Tm {
            tm_sec: 42, // seconds after the minute ~[0-60]
            tm_min: 46, // minutes after the hour ~[0-59]
            tm_hour: 11, // hours after midnight ~[0-23]
            tm_mday: 5, // days of the month ~[1-31]
            tm_mon: 4, // months since January ~[0-11]
            tm_year: 111, // years since 1900
            tm_wday: 4, // days since Sunday ~[0-6]
            tm_yday: 0, // days since January 1 ~[0-365]
            tm_isdst: 0, // Daylight Savings Time flag
            tm_gmtoff: 0, // offset from UTC in seconds
            tm_zone: ~"GMT", // timezone abbreviation
            tm_nsec: 0, // nanoseconds
        });*/

        let path = match _r.request_uri{
            AbsolutePath(ref i) => i.to_str(),
            AbsoluteUri(ref i) => i.to_str(),
            Authority(ref i) => i.to_str(),
            Star => ~"hello"
        };
        println!("{}",path);
        
        w.headers.content_type = Some(headers::content_type::MediaType {
            type_: ~"text",
            subtype: ~"html",
            parameters: ~[]
        });

        let response = render("something");
        w.headers.content_length = Some(response.len());

        let contents = File::open(&Path::new("routes.json")).read_to_end();
        let mut jsonObj = extra::json::from_str(str::from_utf8(contents)).ok();
        println!("{:?}",jsonObj);

        w.write(response.as_bytes());
    }
}

fn render(path: &str) -> ~str {
    let mut response = ~"\
        <html><body><h1>It works!</h1>\n\
        <p>This is the default web page for this server.</p>\n\
        <p>The web server software is running but no content has been added, yet.</p>\n\
        </body></html>\n";
    return response;
    //return File::open(&Path::new(path)).read_to_end();
}

fn main() {
    OxidizeServer.serve_forever();
}