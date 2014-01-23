use http::method::Method;
use servlet_response::ServletResponse;
use std::hashmap::HashMap;
pub fn dispatch_request(url: &str, body: &str, method: &Method) -> ~ServletResponse {
  let sections: ~[&str] = url.split('?').collect();
/*
  let (route,param_string) = match sections {
    [r,p] => (r,p),
    [r] => (r,()),
    _ => fail!("Route must have only one ? character!")
  };
  let params = HashMap::new();
*/
  /*for assignment in param_string.split('&') {
    let*/ 
  let handler = ::router::getHandler(url);
  handler(body)
}
