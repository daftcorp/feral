use servlet::{Request,ResponseBody,TextResponse};
use std::io::{File, io_error, Open, ReadWrite};
use std::str;
use http::status::{Ok, NotFound};
static srv_root: &'static str = ".";
pub fn not_found(req: &Request) -> TextResponse{
  let msg = "Path '" + req.path +"' is not available on this server.";
  TextResponse { code: NotFound, content: msg }
}
pub fn serve_file(req: &Request) -> TextResponse {
    let path = srv_root + req.path;
    let p = Path::new(path);
    if p.exists()
    {
        match File::open(&p) {
            Some(mut f) => TextResponse { code: Ok, content: f.read_to_str() },
            None => not_found(req),
        }
    }
    else {
        not_found(req)
    }
}
