use super::*;

pub struct RequestParameters {
  inner: std::collections::HashMap<String, String>,
}

impl std::convert::TryFrom<Vec<(&str, &str)>> for RequestParameters {
  type Error = error::Error;
  fn try_from(i: Vec<(&str, &str)>) -> std::result::Result<Self, Self::Error> {
    use std::str::FromStr;
    let mut inner = std::collections::HashMap::new();
    for (k, v) in i.into_iter() { inner.insert(k.to_string(), v.to_string()); }
    Ok( RequestParameters { inner, } )
  }
}

impl serde::Serialize for RequestParameters {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where S: serde::Serializer
  {
    use serde::ser::SerializeMap;
    let mut s = serializer.serialize_map(Some(self.inner.len()))?;
    for (k, v) in self.inner.iter() { s.serialize_entry(k, v)?; }
    s.end()
  }
}

