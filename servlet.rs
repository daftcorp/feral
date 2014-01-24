use extra::serialize::{Encodable,Encoder};
use std::hashmap::HashMap;
use http::headers::content_type::MediaType;
use http::method::Method;
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};
pub struct Response {
  code : uint,
  response : Option<~str>,
}
pub struct Request<'a> {
  priv body: &'a str,
  path: ~str,
  params: HashMap<~str,~str>,
  hash : Option<~str>,
  method: &'a Method,
}
impl<'a> Request<'a> {
  pub fn new<'a>(r:&'a ::http::server::Request) -> ~Request<'a> {
    let defroute = ~"/";
    let url: &~str = match r.request_uri {
      AbsolutePath(ref url)  => url,
      _ => &defroute,
    };
    let sections: ~[&str] = url.split('#').collect();
    let (lhs,hash) = match sections {
      [r,h] => (r,Some(h.to_owned())),
      [r] => (r,None),
      _ => (sections[0],None),
    };
    let full_path: ~[&str] = lhs.split('?').collect();
    let (path,qp) = match full_path {
      [r,q] => (r.to_owned(),q),
      [r] => (r.to_owned(),""),
      _ => fail!("asdas"),
    };
    let mut params = HashMap::new();
    for assignment in qp.split('&') {
      let kv_pair: ~[&str] = assignment.split('=').collect();
      let (k,v) = match kv_pair {
        [k,v] => (k,v),
        [k] => (k,""),
        _ => fail!("quizblorg"),
      };
      params.insert(k.to_owned(),v.to_owned());
    }
    ~Request {
      body : r.body,
      path : path,
      params : params,
      hash : hash,
      method : &r.method,
    }
  }
}
