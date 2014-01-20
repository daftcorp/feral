#[crate_id = "generator"];
extern mod extra;
use extra::json::{Json,Object,String,Error};
use std::io::{File,Path,Reader};
use std::hashmap::HashMap;
struct RouteHandler {
  controller : ~str,
  action : ~str
}
fn main() {
  let contents = File::open(&Path::new("routes.json")).read_to_str();
  let result = extra::json::from_str(contents);
  let root = match result {
    Err(_e) => fail!("Failed to parse JSON!"),
    Ok(Object(o)) => o,
    _ => fail!("Root must be object type!")
  };
  let mut file = File::create(&Path::new("router.rs"));
  let mut table = HashMap::<~str,~RouteHandler>::new();
  for (k,v) in root.iter() {
    let target: ~str = match *v {
      String(s) => s,
      _ => fail!("Route values must be strings!")
    };
    let vals: ~[&str] = target.split('#').collect();
    let handler: ~RouteHandler = match vals {
      [c,a] => &RouteHandler { controller : c.to_owned(), action : a.to_owned()},
      _ => fail!("Handler must contain a '#' token!")
    };
    table.insert(target, handler);
  }
  file.write(bytes!("mod router{\nfnroute(url:~str) => Result<~Response>{\nmatch *str {\n"));
}
