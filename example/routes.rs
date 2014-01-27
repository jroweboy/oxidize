static routes: &'static [Route<'static>] = &[
  ("GET", "/", index),
  ("GET", "/test", index)
];


pub fn index(request: Request) -> Response {
  return Response::new(status::Ok, render(request, "templates/index.html", None));
}