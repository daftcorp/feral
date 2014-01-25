use servlet::{Request,Response,Text,JSON};

#[deriving(Encodable)]
struct FooBar {
  num_foos : int,
  num_bars : int,
}
pub fn index(req: &Request) -> ~Response {
  ~Response { code : 200, body : Text(~"index response") }
}
pub fn create(req: &Request) -> ~Response {
  ~Response { code : 200, body : JSON(FooBar { num_foos: 7, num_bars: 9 }) }
}
