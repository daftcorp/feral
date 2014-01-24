use servlet::{Request,Response};
pub fn not_found(_req: &Request) -> ~Response {
  ~Response { code: 404, response: Some(~"Not Found") }
}
