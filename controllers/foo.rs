#[path="../servlet_response.rs"]
mod servlet_response;
pub fn index(body: ~str) -> ~ServletResponse {
  ~ServletResponse { code : 200, response : None }
}
