use http::method::Method;
use servlet::{Request,ResponseBody};
use std::hashmap::HashMap;
pub fn dispatch_request(sr: ~Request) -> ~ResponseBody {
    ::router::getHandler(sr)
}
