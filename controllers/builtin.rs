use servlet::{Request,Response,Text};
use std::io::{File, io_error, Open, ReadWrite};
use std::str;
static srv_root: &'static str = ".";
pub fn not_found(req: &Request) -> ~Response {
  let msg = "Path '" + req.path +"' is not available on this server.";
  ~Response { code: 404, body: Text(msg) }
}
pub fn serve_file(req: &Request) -> ~Response {
    let path = srv_root + req.path;
    let p = Path::new(path);
    if p.exists()
    {
        match File::open(&p) {
            Some(mut f) => ~Response { code: 200, body: Text(f.read_to_str()) },
            None => not_found(req),
        }
    }
    else {
        not_found(req)
    }
}
