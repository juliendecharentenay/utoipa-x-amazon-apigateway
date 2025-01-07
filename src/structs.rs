use super::*;

pub mod root;
pub mod operation;

fn serialize_ref<S>(serializer: S, v: String) -> std::result::Result<S::Ok, S::Error>
where S: serde::Serializer
{
  use serde::ser::SerializeMap;
  let mut s = serializer.serialize_map(Some(1))?;
  s.serialize_entry("$ref", &v)?;
  s.end()
}
