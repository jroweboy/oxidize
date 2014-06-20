use app::App;
use middleware::MiddleWare;
use std::any::{Any, AnyRefExt};
use request::Request;
use response::Response;
use openssl::crypto;

pub trait Session : MiddleWare {
    /// Currently this exists merely because flask has it. I may change it later
    fn make_null_session() -> NullSession {
        NullSession
    }
    /// This function only exists since Flask has it. 
    fn is_null_session(session: &Session) -> bool {
        (&session as &Any).is::<NullSession>()
    }

    // TODO: Implement these as needed
    // fn get_cookie_domain(&self, app: &App) -> String;

    // fn get_cookie_path(&self, app: &App) -> String;

    // fn get_cookie_httponly(&self, app: &App) -> String;
    // and a few more i don't feel like implmenting right now

    /// This method has to be implemented and must either return `None`
    /// in case the loading failed because of a configuration error or an
    /// instance of a session object which implements a dictionary like
    /// interface + the methods and attributes on :class:`SessionMixin`.
    fn open_session(&App, &mut Request) -> Option<Self>;

    fn save_session(&mut self, &App, &mut Response);
}

// maybe move these to their own mods and reexport them here?
pub struct NullSession;

pub struct SecureCookieSession;

impl Session for NullSession {
    fn open_session(app: &App, req: &mut Request) -> Option<NullSession> {
        Some(NullSession)
    }
    fn save_session(&mut self, app: &App, res: &mut Response) {
        // Do nothing. 
    }
}

impl MiddleWare for NullSession {
    fn before(&self, req: &mut Request) {

    }
    fn after(&self, res: &mut Response) {

    }
}

pub struct URLSafeTimedSerializer {
    salt: &'static str,
    /// the hash function to use for the signature. The default is sha1
    digest_method:  crypto::hash::HashType,
    // /// the name of the key derivation. The default is hmac.
    // key_derivation:  "hmac"
    // /// A python serializer for the payload. The default is a compact
    // /// JSON derived serializer with support for some extra Python types
    // /// such as datetime objects or tuples.
    // serializer = session_json_serializer
    // session_class = SecureCookieSession
}

impl URLSafeTimedSerializer {
    /// TODO: Add these as Options in the constructor
    fn new() -> URLSafeTimedSerializer {
        URLSafeTimedSerializer {
            salt: "cookie-session",
            digest_method: crypto::hash::SHA1,
        }
    }

    // fn loads(val: &str, )
}

        // if not app.secret_key:
        //     return None
        // signer_kwargs = dict(
        //     key_derivation=self.key_derivation,
        //     digest_method=self.digest_method
        // )
        // return URLSafeTimedSerializer(app.secret_key, salt=self.salt,
        //                               serializer=self.serializer,
        //                               signer_kwargs=signer_kwargs)

impl Session for SecureCookieSession {
    fn open_session(app: &App, req: &mut Request) -> Option<SecureCookieSession> {
        // let s = self.get_signing_serializer(app)
        // if s.is_none() {
        //     return None;
        // }
        // val = request.cookies.get(app.conf.session_cookie_name)
        // if not val:
        //     return self.session_class()
        // max_age = total_seconds(app.conf.permanent_session_lifetime)
        // try:
        //     data = s.loads(val, max_age=max_age)
        //     return self.session_class(data)
        // except BadSignature:
        //     None
        None
    }
    fn save_session(&mut self, app: &App, res: &mut Response) {
        // Do nothing. 
    }
}

impl MiddleWare for SecureCookieSession {
    fn before(&self, req: &mut Request) {

    }
    fn after(&self, res: &mut Response) {

    }
}