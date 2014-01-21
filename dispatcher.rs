mod dispatcher {
  mod router;
  pub fn dispatch_request(url: &str, body: &str) -> ~ServletResponse {
    let handler = router.getHandler(url);
    handler(body)
  }
}
