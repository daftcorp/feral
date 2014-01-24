use http::method::Method;
use servlet::{Request,Response};
use std::hashmap::HashMap;
pub fn dispatch_request(sr: ~Request) -> ~Response {
  let handler = ::router::getHandler(sr);
  handler(sr)
}
