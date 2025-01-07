use super::*;

#[derive(serde::Serialize)]
#[serde(rename_all="camelCase")]
pub struct RequestValidator {
  pub validate_request_body: bool,
  pub validate_request_parameters: bool,
}

pub struct NamedRequestValidators {
  pub name: String,
  pub request_validator: RequestValidator,
}
impl serde::Serialize for NamedRequestValidators {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where S: serde::Serializer
  {
    use serde::ser::SerializeMap;
    let mut s = serializer.serialize_map(Some(1))?;
    s.serialize_entry(
      &self.name,
      &self.request_validator,
    )?;
    s.end()
  }
}

