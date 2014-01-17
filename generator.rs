#[crate_id = "generator"];
extern mod extra;
use extra::json::{Json,Object,String,Error};
use std::io::{File,Path,Reader};
fn main() {
  let contents = File::open(&Path::new("routes.json")).read_to_str();
  let result = extra::json::from_str(contents);
  let root = match result {
    Err(_e) => fail!("Failed to parse JSON!"),
    Ok(Object(o)) => o,
    _ => fail!("Root must be object type!")
  };
  let myblock = root.iter().map( |(k,v)| k).fold(~"",|acc,key| acc + *key + ~", ");
  println!("{}", myblock);
}
