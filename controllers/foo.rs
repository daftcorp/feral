use servlet::{Request,TextResponse,JSONResponse};
use http::status::Ok;

#[deriving(Encodable)]
struct FooBar {
  num_foos : int,
  num_bars : int,
}
pub fn index(req: &Request) -> TextResponse {
  TextResponse { code : Ok, content : ~"index response" }
}
pub fn create(req: &Request) -> JSONResponse<FooBar> {
  JSONResponse { code : Ok, content : FooBar { num_foos: 7, num_bars: 9 } }
}
