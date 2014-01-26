extern mod extra;
use extra::json::{Object,String};
use std::io::File;
use std::io::fs;
use std::io;
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
    if !controller_exists(handler.controller) {
        fail!(format!("Controller not found: {}",handler.controller));
    }
    modules.insert(handler.controller.to_owned());
  }
  modules.insert(~"builtin");
  file.write(bytes!("use servlet::{Request,ResponseBody};\nmod controllers {\n"));
  file.write(modules.iter().fold(~"",|memo,con| memo + "pub mod "+*con+";\n").as_bytes());
  file.write(bytes!("}\npub fn getHandler(sr: &Request) -> ~ResponseBody {\nmatch sr.path {\n"));
  for (path, handler) in table.iter() {
    file.write(format!("~\"{}\" => ~controllers::{}::{}(sr) as ~ResponseBody,\n",*path, handler.controller, handler.action).as_bytes());
  }
  file.write("_ => ~controllers::builtin::not_found(sr) as ~ResponseBody,\n}\n}".as_bytes());
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

fn controller_exists(cname: &str) -> bool {
    io::result(|| fs::stat(&Path::new(format!("./controllers/{}.rs",cname)))).is_ok()
}
