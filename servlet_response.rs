use extra::serialize::Encodable;
pub struct ServletResponse {
  code : uint,
  response : Option<~Encodable>
}
