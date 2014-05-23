// use http::status::Status;

/// TODO: Very lack luster at the moment but yeah. I'm not sure what I want in here yet anyway
/// I am thinking that I want to abstract away the ResponeWriter since once data is written to the
/// ResponseWriter, no middleware can act on it, but still have the ResponseWriter as part of the
/// Response so that people that don't want middleware and want the data written fast can just use that

// pub struct Response {
//   content : ~str,
//   status: Status
// }
