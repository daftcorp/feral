use std::str::from_utf8_owned;
use extra::serialize::{Encodable,Encoder};
use extra::json;
use std::hashmap::HashMap;
use http::headers::content_type::MediaType;
use http::method::Method;
use http::server::ResponseWriter;
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};
use http::status::{Ok, Status};

// FIXME: no code/handling and bad content-type hinting
pub trait ResponseBody {
    fn get_code(&self) -> Status;
    fn process(&self) -> (~str, MediaType);
    fn write_to<'a>(&'a self,w : &mut ResponseWriter){
        let (content, mt) = self.process();
        // FIXME: this is cheating, for now, and just using the
        // write_content_auto() path for the response.. i could
        // see the original structure was probably leaning towards
        // write(), but i just wanted to put this to bed.. going back
        // to write would imply probably changing process's signature,
        // above, to use &'a [u8] (requiring parameterizing on lifetime)
        // or using ~[u8]
        w.status = self.get_code();
        w.write_content_auto(mt, content);
    }
}

pub struct JSONResponse<TJSON> {
    code: Status,
    content : TJSON
}

impl<'a, TJSON: Encodable<json::Encoder<'a>>> ResponseBody for JSONResponse<TJSON> {
    fn get_code(&self) -> Status { self.code.clone() }
    fn process(&self) -> (~str, MediaType) {
        let content = match from_utf8_owned(
                json::Encoder::buffer_encode(&self.content)) {
            Some(c) => c,
            None => ~"" // FIXME: handle failure?
        };
        (content.clone(), MediaType(~"application", ~"json", ~[]))
    }
}

pub struct TextResponse {
    code: Status,
    content: ~str
}

impl ResponseBody for TextResponse {
    fn get_code(&self) -> Status { self.code.clone() }
    fn process(&self) -> (~str, MediaType) {
        // FIXME: probably want something more flexible for specifying the
        // content type of the resp.. text/html probably isn't at-all the
        // the right answer.
        (self.content.clone(), MediaType(~"text", ~"html", ~[]))
    }
}

pub struct EmptyResponse;

impl ResponseBody for EmptyResponse {
    fn get_code(&self) -> Status { Ok }
    fn process(&self) -> (~str, MediaType) {
        // FIXME: same as above
        (~"", MediaType(~"text", ~"html", ~[]))
    }
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
