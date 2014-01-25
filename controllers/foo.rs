use servlet::{Request,Response,Text};
pub fn index(req: &Request) -> ~Response {
  ~Response { code : 200, body : Text(~"index response") }
}
pub fn create(req: &Request) -> ~Response {
  ~Response { code : 200, body : Text(~"create response") }
}
