extern mod extra;
use extra::json::{Object,String};
use std::io::{File};
use std::hashmap::{HashMap,HashSet};

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
  let table = toMap(root);
  let mut file = File::create(&Path::new("router.rs"));
  let mut modules = HashSet::new();
  for (_k,handler) in table.iter() {
    modules.insert(handler.controller.to_owned());
  }
  file.write(modules.iter().map(|m|format!("mod controllers::{};\n",*m)).fold(~"",|memo,con| memo + con).as_bytes());
  file.write(bytes!("mod servlet_response;\npub fn getHandler(url:&str) -> fn(~str) -> ~ServletResponse {\nmatch *str {\n"));
  for (path, handler) in table.iter() {
    file.write(format!("\"{}\" => controllers::{}::{},\n",*path, handler.controller, handler.action).as_bytes());
  }
  file.write("}\n}".as_bytes());
}

fn toMap(root: ~Object) -> ~HashMap<~str,RouteHandler> {
  let mut table = HashMap::<~str,RouteHandler>::new();
  for (k,v) in root.iter() {
    let target: ~str = match *v {
      String(ref s) => s.to_owned(),
       _ => fail!("Route values must be strings!")
    };
    let vals: ~[&str] = target.split('#').collect();
    let handler = match vals {
      [c,a] => RouteHandler { controller : c.to_owned(), action : a.to_owned()},
       _ => fail!("Handler must contain a '#' token!")
    };
    table.insert(k.to_owned(), handler);
  }
  ~table
}
