use servlet::{Request,Response};
pub fn index(req: &Request) -> ~Response {
  ~Response { code : 200, response : Some(~"index response") }
}
pub fn create(req: &Request) -> ~Response {
  ~Response { code : 200, response : Some(~"create response") }
}
