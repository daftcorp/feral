use servlet_response::ServletResponse;
pub fn index(body: &str) -> ~ServletResponse {
  ~ServletResponse { code : 200, response : None }
}
pub fn create(body: &str) -> ~ServletResponse {
  ~ServletResponse { code : 200, response : None }
}
