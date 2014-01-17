#[crate_id = "feral"];

extern mod extra;
extern mod http;

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::Writer;

use http::server::{Config, Server, Request, ResponseWriter};
use http::server::request::{Star, AbsoluteUri, AbsolutePath, Authority};
use http::headers::content_type::MediaType;
use std::io::{File, io_error, Open, ReadWrite};

#[deriving(Clone)]
struct FeralServer;

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
      let path = match r.request_uri {
         AbsolutePath(ref url)  => url.slice_from(1).to_owned(),
         _ => ~"README.md"
      };
      println!("Got path: {}",path);
      let contents = File::open(&Path::new(path)).read_to_end();
      w.write(contents);
    }
}

fn main() {
    FeralServer.serve_forever();
}
