

pub trait Renderer {
	fn render(&self, file_name: &str) -> ~str;
}