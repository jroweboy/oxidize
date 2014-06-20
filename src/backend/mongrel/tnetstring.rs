use std::f64;
use std::collections::HashMap;
use std::io;
use std::num::strconv;
use std::str;
use std::from_str::from_str;
use std::from_str::FromStr;

pub struct Error {
    msg: String,
}

fn io_error_to_error(io: io::IoError) -> Error {
    Error { msg: format!("{}", io) }
}

/// Represents a TNetString value.
pub enum TNetString {
    Str(Vec<u8>),
    Int(int),
    Float(f64),
    Bool(bool),
    Null,
    Map(Map),
    Vec(Vec<TNetString>),
}

pub type Map = HashMap<Vec<u8>, TNetString>;

/// Serializes a TNetString value into a `Writer`.
pub fn to_writer(writer: &mut Writer, tnetstring: &TNetString) -> io::IoResult<()> {
    fn write_str(wr: &mut Writer, s: &[u8]) -> io::IoResult<()> {
        try!(write!(wr, "{}:", s.len()));
        try!(wr.write(s));
        try!(write!(wr, ","));
        Ok(())
    }

    match *tnetstring {
        Str(ref s) => {
            write_str(writer, s.as_slice())
        }
        Int(i) => {
            let s = i.to_str();
            try!(write!(writer, "{}:{}\\#", s.len(), s));
            Ok(())
        }
        Float(f) => {
            let s = f64::to_str_digits(f, 6u);
            try!(write!(writer, "{}:{}^", s.len(), s));
            Ok(())
        }
        Bool(b) => {
            let s = b.to_str();
            try!(write!(writer, "{}:{}!", s.len(), s));
            Ok(())
        }
        Map(ref m) => {
            let mut wr = io::MemWriter::new();
            for (key, value) in m.iter() {
                try!(write_str(&mut wr as &mut Writer, key.as_slice()));
                try!(to_writer(&mut wr as &mut Writer, value));
            }
            let payload = wr.unwrap();
            try!(write!(writer, "{}:", payload.len()));
            try!(writer.write(payload.as_slice()));
            try!(write!(writer, "}}"));
            Ok(())
        }
        Vec(ref v) => {
            let mut wr = io::MemWriter::new();
            for e in v.iter() {
                try!(to_writer(&mut wr as &mut Writer, e))
            }
            let payload = wr.unwrap();
            try!(write!(writer, "{}:", payload.len()));
            try!(writer.write(payload.as_slice()));
            try!(write!(writer, "]"));
            Ok(())
        }
        Null => {
            try!(write!(writer, "0:~"));
            Ok(())
        }
    }
}

/// Serializes a TNetString value into a byte string.
pub fn to_bytes(tnetstring: &TNetString) -> io::IoResult<Vec<u8>> {
    let mut wr = io::MemWriter::new();
    try!(to_writer(&mut wr as &mut Writer, tnetstring));
    Ok(wr.unwrap())
}

/// Serializes a TNetString value into a string.
// impl ToStr for TNetString {
//     fn to_str(&self) -> String {
//         str::from_utf8_owned(to_bytes(self).unwrap()).unwrap()
//     }
// }

/// Deserializes a `TNetString` value from a `Reader`.
pub fn from_reader<R: Reader + Buffer>(rdr: &mut R) -> Result<Option<TNetString>, Error> {
    let mut ch = match rdr.read_byte() {
        Ok(ch) => ch,
        Err(ref err) if err.kind == io::EndOfFile => { return Ok(None); }
        Err(err) => { return Err(io_error_to_error(err)); }
    };
    let mut len = 0u;

    // Note that netstring spec explicitly forbids padding zeros.
    // If the first char is zero, it must be the only char.
    if ch < '0' as u8 || ch > '9' as u8 {
        fail!("Not a TNetString: invalid or missing length prefix");
    } else if ch == '0' as u8 {
        ch = match rdr.read_byte() {
            Ok(ch) => ch,
            Err(err) => { return Err(io_error_to_error(err)); }
        };
    } else {
        loop {
            len = (10u * len) + ((ch as uint) - ('0' as uint));

            ch = match rdr.read_byte() {
                Ok(ch) => ch,
                Err(err) => { return Err(io_error_to_error(err)); }
            };

            if ch < '0' as u8 || ch > '9' as u8 {
                break;
            }
        }
    }

    // Validate end-of-length-prefix marker.
    if ch != ':' as u8 {
        return Err(Error { msg: "Not a TNetString: missing length prefix".to_string() });
    }

    // Read the data plus terminating type tag.
    let payload = match rdr.read_exact(len) {
        Ok(payload) => payload,
        Err(err) => { return Err(io_error_to_error(err)); }
    };

    if payload.len() != len {
        return Err(Error { msg: "Not a TNetString: invalid length prefix".to_string() });
    }

    let ch = match rdr.read_char() {
        Ok(ch) => ch,
        Err(err) => { return Err(io_error_to_error(err)); }
    };

    let value = match ch {
        '#' => {
            let v = strconv::from_str_bytes_common(payload.as_slice(), 10, true, false, false,
                                                   strconv::ExpNone, false, false);
            match v {
                Some(v) => Some(Int(v)),
                None => { return Err(Error { msg: "invalid integer".to_string() }); }
            }
        }
        '}' => Some(Map(try!(parse_map(payload.as_slice())))),
        ']' => Some(Vec(try!(parse_vec(payload.as_slice())))),
        '!' => {
            // this is a very ugly way to convert the vec "true" or false into a Some(Bool)
            match str::from_utf8_owned(payload).and_then(
                    // and_then needs a Result, but from_str returns a Option so we convert to a Result
                    |s| match from_str::<bool>(s.as_slice()) { 
                        // check to see if the Result is there
                        Some(a) => Ok(a),
                        None => Err(Vec::new())
                    } 
                ){
                Ok(b) => Some(Bool(b)),
                _ => {
                    return Err(Error { msg: "invalid bool".to_string() }); 
                }
            }
        }
        '^' => {
            let v = strconv::from_str_bytes_common(payload.as_slice(), 10u, true, true, true,
                                                   strconv::ExpDec, false, false);

            match v {
                Some(v) => Some(Float(v)),
                None => { return Err(Error { msg: "invalid float".to_string() }); }
            }
        }
        '~' => {
            if payload.is_empty() {
                Some(Null)
            } else {
                return Err(Error { msg: "invalid null".to_string() });
            }
        }
        ',' => {
            Some(Str(payload))
        }
        ch => {
            return Err(Error { msg: format!("Invalid payload type: {}", ch) });
        }
    };

    Ok(value)
}

fn parse_vec(data: &[u8]) -> Result<Vec<TNetString>, Error> {
    if data.is_empty() { return Ok(Vec::new()); }

    let mut result = Vec::new();
    let mut rdr = io::BufReader::new(data);

    loop {
        match try!(from_reader(&mut rdr)) {
            Some(value) => { result.push(value); }
            None => { return Ok(result); }
        }
    }
}

fn parse_pair<R: Reader + Buffer>(rdr: &mut R) -> Result<Option<(Vec<u8>, TNetString)>, Error> {
    match try!(from_reader(rdr)) {
        Some(Str(key)) => {
            match try!(from_reader(rdr)) {
                Some(value) => Ok(Some((key, value))),
                None => { return Err(Error { msg: "invalid TNetString".to_string() }); }
            }
        }
        Some(_) => Err(Error { msg: "Keys can only be strings.".to_string() }),
        None => Ok(None),
    }
}

fn parse_map(data: &[u8]) -> Result<HashMap<Vec<u8>, TNetString>, Error> {
    let mut result = HashMap::new();
    let mut rdr = io::BufReader::new(data);

    loop {
        match try!(parse_pair(&mut rdr)) {
            Some((key, value)) => { result.insert(key, value); }
            None => { return Ok(result); }
        }
    }
}

/// Deserializes a TNetString value from a byte string.
pub fn from_bytes<'a>(data: &'a [u8]) -> Result<(Option<TNetString>, io::BufReader<'a>), Error> {
    let mut rdr = io::BufReader::new(data);
    let tnetstring = try!(from_reader(&mut rdr));
    Ok((tnetstring, rdr))
}

/// Deserializes a TNetString value from a string.
impl FromStr for TNetString {
    fn from_str<'a>(data: &'a str) -> Option<TNetString> {
        match from_bytes(data.as_bytes()) {
            Ok(s) => s.val0(),
            Err(_) => None,
        }
    }
}

/// Test the equality between two TNetString values
impl PartialEq for TNetString {
    fn eq(&self, other: &TNetString) -> bool {
        match (self, other) {
            (&Str(ref s0), &Str(ref s1)) => s0 == s1,
            (&Int(i0), &Int(i1)) => i0 == i1,
            (&Float(f0), &Float(f1)) => f0 == f1,
            (&Bool(b0), &Bool(b1)) => b0 == b1,
            (&Null, &Null) => true,
            (&Map(ref d0), &Map(ref d1)) => {
                if d0.len() == d1.len() {
                    for (k0, v0) in d0.iter() {
                        // XXX send_map::linear::LinearMap has find_ref, but
                        // that method is not available for HashMap.
                        let result = match d1.find(k0) {
                            Some(v1) => v0 == v1,
                            None => false,
                        };
                        if !result { return false; }
                    }
                    true
                } else {
                    false
                }
            }
            (&Vec(ref v0), &Vec(ref v1)) => {
                v0.eq(v1)
            },
            _ => false
        }
    }

    fn ne(&self, other: &TNetString) -> bool { !self.eq(other) }
}

#[cfg(test)]
mod tests {
    use std::f64;
    use std::hashmap::HashMap;
    use std::rand;
    use std::rand::Rng;
    use std::vec;

    use super::TNetString;
    use super::{Str, Int, Float, Bool, Null, Map, Vec};
    use super::{from_bytes, to_bytes};
    use super::from_str;

    // Tests inspired by https://github.com/rfk/TNetString.

    fn test(s: &str, expected: &TNetString) {
        let (actual, rest) = from_str(s).unwrap();
        assert!(actual.is_some());
        assert!(rest.eof());

        let actual = actual.unwrap();
        assert_eq!(actual, *expected);
        assert_eq!(expected.to_str(), s.to_owned());
    }

    #[test]
    fn test_format() {
        test("11:hello world,", &Str(("hello world".to_string()).into_bytes()));
        test("0:}", &Map(HashMap::new()));
        test("0:]", &Vec(Vec::new()));

        let mut d = HashMap::new();
        d.insert(("hello").into_bytes(),
                Vec(vec![
                    Int(12345678901),
                    Str(("this".to_string()).into_bytes()),
                    Bool(true),
                    Null,
                    Str(("\x00\x00\x00\x00".to_string()).into_bytes())
                ]));

        test("51:5:hello,39:11:12345678901#4:this,4:true!0:~4:\x00\x00\x00\
               \x00,]}", &Map(d));

        test("5:12345#", &Int(12345));
        test("12:this is cool,", &Str(("this is cool".to_string()).into_bytes()));
        test("0:,", &Str((String::new()).into_bytes()));
        test("0:~", &Null);
        test("4:true!", &Bool(true));
        test("5:false!", &Bool(false));
        test("10:\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00,",
            &Str(("\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00".to_string()).into_bytes()));
        test("24:5:12345#5:67890#5:xxxxx,]",
            &Vec(vec![
                Int(12345),
                Int(67890),
                Str(("xxxxx".to_string()).into_bytes())]));
        test("18:3:0.1^3:0.2^3:0.4^]",
           &Vec(vec![Float(0.1), Float(0.2), Float(0.4)]));
        test("243:238:233:228:223:218:213:208:203:198:193:188:183:178:173:\
               168:163:158:153:148:143:138:133:128:123:118:113:108:103:99:95:\
               91:87:83:79:75:71:67:63:59:55:51:47:43:39:35:31:27:23:19:15:\
               11:hello-there,]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]\
               ]]]]",
            &Vec(
                vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(
                vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(
                vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(
                vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(
                vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(
                vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(vec![Vec(
                vec![Vec(vec![Vec(vec![
                    Str(("hello-there".to_string()).into_bytes())
                ])])])])])])])])])])])])])])])])])])])])])])])])])])])])
                ])])])])])])])])])])])])])])])])])])])])])])]));
    }

    #[test]
    fn test_random() {
        fn get_random_object(rng: &mut rand::StdRng, depth: u32) -> TNetString {
            if rng.gen_range(depth, 10u32) <= 4u32 {
                if rng.gen_range(0u32, 1u32) == 0u32 {
                    let n = rng.gen_range(0u32, 10u32);
                    Vec(vec::from_fn(n as uint, |_i|
                        get_random_object(rng, depth + 1u32)
                    ))
                } else {
                    let mut d = HashMap::new();

                    let mut i = rng.gen_range(0u32, 10u32);
                    while i != 0u32 {
                        let n = rng.gen_range(0u32, 100u32) as uint;
                        let s = rng.gen_vec(n);
                        d.insert(
                            s,
                            get_random_object(rng, depth + 1u32)
                        );
                        i -= 1u32;
                    }
                    Map(d)
                }
            } else {
                match rng.gen_range(0u32, 5u32) {
                  0u32 => Null,
                  1u32 => Bool(true),
                  2u32 => Bool(false),
                  3u32 => {
                    if rng.gen_range(0u32, 1u32) == 0u32 {
                        Int(rng.next_u32() as int)
                    } else {
                        Int(-rng.next_u32() as int)
                    }
                  }
                  4u32 => {
                    let mut f = rng.gen::<f64>();

                    // Generate a float that can be exactly converted to
                    // and from a string.
                    loop {
                        match FromStr::from_str(f64::to_str_digits(f, 6)) {
                          Some(f1) => {
                            if f == f1 { break; }
                            f = f1;
                          }
                          None => fail!("invalid float")
                        }
                    }

                    if rng.gen_range(0u32, 1u32) == 0u32 {
                        Float(f)
                    } else {
                        Float(-f)
                    }
                  }
                  5u32 => {
                    let n = rng.gen_range(0u32, 100u32) as uint;
                    Str(rng.gen_vec(n))
                  }
                  _ => fail!()
                }
            }
        }

        let mut rng = rand::rng();

        let mut i = 500;
        while i != 0 {
            let v0 = get_random_object(&mut rng, 0u32);
            let bytes = to_bytes(&v0).unwrap();

            match from_bytes(bytes).unwrap() {
                (Some(ref v1), rest) if rest.eof() => {
                    assert!(v0 == *v1)
                },
                _ => fail!("invalid TNetString")
            }
            i -= 1u;
        }
    }
}
