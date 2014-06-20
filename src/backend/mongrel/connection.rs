use std::collections::HashMap;
use zmq;
use std::io;
use std::io::{BufReader, Reader};
use serialize::json;
use std::str;
use backend::mongrel::tnetstring;

pub struct Connection {
    sender_id: Option<String>,
    req_addrs: Vec<String>,
    res_addrs: Vec<String>,
    req: zmq::Socket,
    res: zmq::Socket,
}

pub fn connect(ctx: &mut zmq::Context, sender_id: Option<String>, 
               req_addrs: Vec<String>,res_addrs: Vec<String>) -> Connection {
    let mut req = match ctx.socket(zmq::PULL) {
        Ok(req) => req,
        Err(e) => fail!(e.to_str()),
    };

    for req_addr in req_addrs.iter() {
        match req.connect(req_addr.as_slice()) {
          Ok(()) => { },
          Err(e) => fail!(e.to_str()),
        }
    }

    let mut res = match ctx.socket(zmq::PUB) {
        Ok(res) => res,
        Err(e) => fail!(e.to_str()),
    };

    match sender_id {
        None => { },
        Some(ref sender_id) => {
            match res.set_identity(sender_id.as_bytes()) {
                Ok(()) => { },
                Err(e) => fail!(e.to_str()),
            }
        }
    }

    for res_addr in res_addrs.iter() {
        match res.connect(res_addr.as_slice()) {
            Ok(()) => { },
            Err(e) => fail!(e.to_str()),
        }
    }

    Connection {
        sender_id: sender_id,
        req_addrs: req_addrs,
        res_addrs: res_addrs,
        req: req,
        res: res
    }
}

impl Connection {
    fn req_addrs<'a>(&'a self) -> &'a Vec<String> { &self.req_addrs }
    fn res_addrs<'a>(&'a self) -> &'a Vec<String> { &self.res_addrs }

    pub fn recv(&mut self) -> Result<Request, String> {
        match self.req.recv_msg(0) {
            Err(e) => Err(e.to_str()),
            Ok(msg) => msg.with_bytes(|bytes| parse(bytes)),
        }
    }

    pub fn send(&mut self,
            uuid: &str,
            id: &[String],
            body: &[u8]) -> Result<(), String> {
        let id = id.connect(" ").into_bytes();

        let mut msg = Vec::new();

        msg.push_all(uuid.as_bytes());
        msg.push(' ' as u8);
        msg.push_all(tnetstring::to_bytes(&tnetstring::Str(id)).ok().unwrap().as_slice());
        msg.push(' ' as u8);
        msg.push_all(body);

        match self.res.send(msg.as_slice(), 0) {
          Err(e) => Err(e.to_str()),
          Ok(()) => Ok(()),
        }
    }

    pub fn reply(&mut self, req: &Request, body: &[u8]) -> Result<(), String> {
        //self.send(req.uuid, [copy req.id], body)
        self.send(req.uuid.as_slice(), [req.id.clone()], body)
    }

    pub fn reply_http(&mut self, req: &Request, code: uint, status: &str, 
                      headers: Headers, body: String) -> Result<(), String> {
        let mut res = Vec::new();

        res.push_all((format!("HTTP/1.1 {} ", code)).into_bytes().as_slice());
        res.push_all(status.as_bytes());
        res.push_all("\r\n".as_bytes());
        res.push_all("Content-Length: ".as_bytes());
        res.push_all(body.len().to_str().into_bytes().as_slice());
        res.push_all("\r\n".as_bytes());

        for (key, values) in headers.iter() {
            for value in values.iter() {
                res.push_all(key.as_slice().as_bytes());
                res.push_all(": ".as_slice().as_bytes());
                res.push_all(value.as_slice().as_bytes());
                res.push_all("\r\n".as_slice().as_bytes());
            };
        }
        res.push_all("\r\n".as_bytes());
        res.push_all(body.as_slice().as_bytes());

        self.reply(req, res.as_slice())
    }

    pub fn term (&mut self) {
        self.req.close();
        self.res.close();
    }
}

// TODO: there is no `as_bytes' for String that will return Vec<u8>.
// fn str_as_bytes(s: String) -> Vec<u8> {
//     let s = s.clone();
//     let mut buf: Vec<u8> = unsafe { cast::transmute(s) };
//     buf.pop();
//     buf
// }

pub type Headers = HashMap<String, Vec<String>>;

pub fn Headers() -> Headers {
    HashMap::new()
}

#[deriving(Clone)]
pub struct Request {
    pub uuid: String,
    pub id: String,
    pub path: String,
    pub headers: Headers,
    pub body: Vec<u8>,
    pub json_body: Option<Box<json::Object>>,
}

impl Request {
    pub fn is_disconnect(&self) -> bool {
        match self.json_body {
            None => false,
            Some(ref map) => {
                match map.find(&"type".to_string()) {
                    Some(&json::String(ref typ)) => *typ == "disconnect".to_string(),
                    _ => false,
                }
            }
        }
    }

    pub fn should_close(&self) -> bool {
        match self.headers.find(&"connection".to_string()) {
          None => { },
          Some(conn) => {
            if conn.len() == 1u && conn.get(0).as_slice() == "close" { return true; }
          }
        }

        match self.headers.find(&"VERSION".to_string()) {
          None => false,
          Some(version) => {
            version.len() == 1u && version.get(0).as_slice() == "HTTP/1.0"
          }
        }
    }
}

fn parse(bytes: &[u8]) -> Result<Request, String> {
    parse_reader(&mut BufReader::new(bytes))
}

fn parse_reader(rdr: &mut BufReader) -> Result<Request, String> {
    let uuid = match parse_uuid(rdr) {
        Ok(uuid) => uuid,
        Err(e) => return Err(e),
    };

    let id = match parse_id(rdr) {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    let path = match parse_path(rdr) {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    let headers = match parse_headers(rdr) {
        Ok(headers) => headers,
        Err(e) => return Err(e),
    };

    let body = match parse_body(rdr) {
        Ok(body) => body,
        Err(e) => return Err(e),
    };

    // Extract out the json body if we have it.
    let json_body = match headers.find(&"METHOD".to_string()) {
      None => None,
      Some(method) => {
        if method.len() == 1u && method.get(0).as_slice() == "JSON" {
            match json::from_str(str::from_utf8(body.as_slice()).unwrap()) {
              Ok(json::Object(map)) => Some(map),
              Ok(_) => return Err("json body is not a dictionary".to_string()),
              Err(e) =>
                return Err(format!("invalid JSON string: {}", e)),
            }
        } else { None }
      }
    };

    Ok(Request {
        uuid: uuid,
        id: id,
        path: path,
        headers: headers,
        body: body,
        json_body: json_body
    })
}

// This appears to be fundamentally flawed? It would miss a string if it was like
// "TestEOF" since it wouldn't hit a space before the EOF? Maybe that case never happens...
fn read_str(rdr: &mut BufReader) -> Option<String> {
    let mut s = String::new();

    while !rdr.eof() {
        let ch = rdr.read_char();
        match ch {
            Ok(c) => {
                if c == ' ' {
                    return Some(s);
                } else {
                    s.push_char(c);
                }
            }
            Err(e) => { return None; }
        }
    }

    None
}

fn parse_uuid(rdr: &mut BufReader) -> Result<String, String> {
    match read_str(rdr) {
        Some(s) => Ok(s),
        None => Err("invalid sender uuid".to_string()),
    }
}

fn parse_id(rdr: &mut BufReader) -> Result<String, String> {
    match read_str(rdr) {
        Some(s) => Ok(s),
        None => Err("invalid connection id".to_string()),
    }
}

fn parse_path(rdr: &mut BufReader) -> Result<String, String> {
    match read_str(rdr) {
        Some(s) => Ok(s),
        None => Err("invalid path".to_string()),
    }
}

fn parse_headers(rdr: &mut BufReader) -> Result<Headers, String> {
    let tns = match tnetstring::from_reader(rdr) {
        Err(e) => return Err("empty headers".to_string()),
        Ok(tns) => tns,
    };

    match tns.unwrap() {
        tnetstring::Map(map) => parse_tnetstring_headers(map),

        // Fall back onto json if we got a string.
        tnetstring::Str(bytes) => {
            match json::from_str(str::from_utf8(bytes.as_slice()).unwrap()) {
                Err(e) => return Err(e.to_str()),
                Ok(json::Object(map)) => parse_json_headers(map),
                Ok(_) => Err("header is not a dictionary".to_string()),
            }
        }

        _ => Err("invalid header".to_string()),
    }
}

fn parse_tnetstring_headers(map: tnetstring::Map) -> Result<Headers, String> {
    let mut headers = HashMap::new();

    for (key, value) in map.iter() {
        let key = str::from_utf8(key.as_slice()).unwrap().to_string();
        let mut values = match headers.pop(&key) {
            Some(values) => values,
            None => Vec::new(),
        };

        match value {
            &tnetstring::Str(ref v) => values.push(str::from_utf8(v.as_slice()).unwrap().to_string()),
            &tnetstring::Vec(ref vs) => {
                for v in vs.iter() {
                    match v {
                        &tnetstring::Str(ref v) =>
                            values.push(str::from_utf8(v.as_slice()).unwrap().to_string()),
                        _ => return Err("header value is not a string".to_string()),
                    }
                }
            },
            _ => return Err("header value is not string".to_string()),
        }

        headers.insert(key, values);
    }

    Ok(headers)
}

fn parse_json_headers<'a>(map: &'a json::Object) -> Result<Headers, String> {
    let mut headers = HashMap::new();

    for (key, value) in map.iter() {
        let mut values = match headers.pop(key) {
            Some(values) => values,
            None => Vec::new(),
        };

        match value {
            &json::String(ref v) => values.push(v.clone()),
            &json::List(ref vs) => {
                for v in vs.iter() {
                    match v {
                        &json::String(ref v) => values.push(v.clone()),
                        _ => return Err("header value is not a string".to_string()),
                    }
                }
            }
            _ => return Err("header value is not string".to_string()),
        }

        headers.insert(key.clone(), values);
    }

    Ok(headers)
}

fn parse_body(rdr: &mut BufReader) -> Result<Vec<u8>, String> {
    match tnetstring::from_reader(rdr).ok() {
        None => Err("empty body".to_string()),
        Some(tns) => {
            match tns.unwrap() {
                tnetstring::Str(body) => Ok(body),
                _ => Err("invalid body".to_string()),
            }
        }
    }
}

// #[test]
// fn test() {
//     let ctx = zmq::Context::new();

//     let mut connection = connect(ctx,
//         Some(~"F0D32575-2ABB-4957-BC8B-12DAC8AFF13A"),
//         ~[~"tcp://127.0.0.1:9998"],
//         ~[~"tcp://127.0.0.1:9999"]);

//     connection.term();
// }

// #[test]
// fn test_request_parse() {
//     let request = parse(
//         bytes!("abCD-123 56 / 13:{\"foo\":\"bar\"},11:hello world,")
//     ).unwrap();

//     assert!(request.uuid == ~"abCD-123");
//     assert!(request.id == ~"56");
//     assert!(request.headers.len() == 1u);
//     let value = match request.headers.find(&~"foo") {
//         Some(header_list) => header_list[0u].clone(),
//         None => ~"",
//     };
//     assert!(value == ~"bar");
//     assert!(request.body == (~"hello world").into_bytes());
// }
