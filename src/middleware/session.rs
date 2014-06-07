use app::App;
use middleware::MiddleWare;
use std::any::{Any, AnyRefExt};

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
    fn open_session(app: &App, &mut Request) -> Option<NullSession> {
        Some(NullSession)
    }
    fn save_session(&mut self, app: &App, res: &mut Response) {
        // Do nothing. 
    }
}

impl Session for SecureCookieSession {
    fn open_session(app: &App, &mut Request) -> Option<SecureCookieSession> {
        s = self.get_signing_serializer(app)
        if s is None:
            return None
        val = request.cookies.get(app.session_cookie_name)
        if not val:
            return self.session_class()
        max_age = total_seconds(app.permanent_session_lifetime)
        try:
            data = s.loads(val, max_age=max_age)
            return self.session_class(data)
        except BadSignature:
            return self.session_class()
    }
    fn save_session(&mut self, app: &App, res: &mut Response) {
        // Do nothing. 
    }
}
