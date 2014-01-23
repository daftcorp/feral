use servlet_response::ServletResponse;
use http::method::Method;
pub fn not_found(body: &str) -> ~ServletResponse {
  ~ServletResponse { code: 404, response: Some(~"Not Found") }
}
