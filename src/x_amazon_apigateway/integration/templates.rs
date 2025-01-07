use super::*;

pub struct Templates {
  inner: std::collections::HashMap<mime::Mime, String>,
}

impl std::convert::TryFrom<Vec<(&str, &str)>> for Templates {
  type Error = error::Error;
  fn try_from(i: Vec<(&str, &str)>) -> std::result::Result<Self, Self::Error> {
    let mut inner = std::collections::HashMap::new();
    for (m, v) in i.into_iter() { inner.insert(m.parse()?, v.to_string()); }
    Ok( Templates { inner, } )
  }
}

impl std::convert::TryFrom<Vec<(mime::Mime, &str)>> for Templates {
  type Error = error::Error;
  fn try_from(i: Vec<(mime::Mime, &str)>) -> std::result::Result<Self, Self::Error> {
    let mut inner = std::collections::HashMap::new();
    for (m, v) in i.into_iter() { inner.insert(m, v.to_string()); }
    let r = Templates { inner, };
    Ok(r)
  }
}

/*
impl std::convert::TryFrom<&[(mime::Mime, &str)]> for Templates {
  type Error = error::Error;
  fn try_from(i: &[(mime::Mime, &str)]) -> std::result::Result<Self, Self::Error> {
    let mut inner = std::collections::HashMap::new();
    for (m, v) in i.iter() { inner.insert(m.clone(), v.to_string()); }
    let r = Templates { inner, };
    Ok(r)
  }
}
*/

impl serde::Serialize for Templates {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where S: serde::Serializer
  {
    use serde::ser::SerializeMap;
    let mut s = serializer.serialize_map(Some(self.inner.len()))?;
    for (k, v) in self.inner.iter() {
      s.serialize_entry(k.as_ref(), v)?;
    }
    s.end()
  }
}

