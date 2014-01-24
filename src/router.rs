
pub trait Router {
	fn route(&self, path: &str) -> ~str;
}