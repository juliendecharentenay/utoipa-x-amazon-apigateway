use super::*;

/// Pendent to [integration.responses](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-integration-responses.html)
pub struct Responses {
  inner: std::collections::HashMap<String, Response>,
}

impl std::convert::TryFrom<Vec<(&str, Response)>> for Responses {
  type Error = error::Error;
  fn try_from(i: Vec<(&str, Response)>) -> std::result::Result<Self, Self::Error> {
    let mut inner = std::collections::HashMap::new();
    for (k, v) in i.into_iter() { inner.insert(k.to_string(), v); }
    Ok(Responses { inner, })
  }
}

impl serde::Serialize for Responses {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where S: serde::Serializer
  {
    self.inner.serialize(serializer)
  }
}

