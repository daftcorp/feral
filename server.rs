extern mod extra;
extern mod http;
use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::Writer;

use http::server::{Config, Server, Request, ResponseWriter};
use http::headers::content_type::MediaType;

mod dispatcher;
mod router;
mod servlet;


#[deriving(Clone)]
pub struct FeralServer;

impl Server for FeralServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
    }

    fn handle_request(&self, r: &Request, w: &mut ResponseWriter) {
        w.headers.content_type = Some(MediaType {
            type_: ~"text",
            subtype: ~"plain",
            parameters: ~[(~"charset", ~"UTF-8")]
        });
      let req = servlet::Request::new(r);
      println!("Got req: {:?}",req);
      let sr = dispatcher::dispatch_request(req);
      let contents = match sr.response {
        Some(s) => s,
        None => ~"",
      };
      w.write(contents.as_bytes());
    }
}


fn main() {
    FeralServer.serve_forever();
}
