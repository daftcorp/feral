mod dispatcher {
  mod router;
  pub fn dispatch_request(url: &str, body: &str) -> ~ServletResponse {
    let sections = url.split('?').collect();
    let (route,param_string) = match sections {
      [r,p] => (r,p),
      [r] => (r,()),
      _ => fail!("Route must have only one ? character!")
    };
    let params = HashMap::new();
    for assignment in param_string.split('&') {
      let 
    let handler = router.getHandler(url);
    handler(body)
  }
}
