use extra::serialize::{Encodable,Encoder};
pub struct ServletResponse {
  code : uint,
  response : Option<~str>
}
