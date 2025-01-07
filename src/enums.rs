
#[derive(strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum PassthroughBehavior {
  WhenNoTemplates,
  WhenNoMatch,
  Never,
}

#[derive(strum::Display, strum::EnumString)]
#[strum(serialize_all = "Train-Case")]
#[derive(Hash, PartialEq, Eq)]
pub enum ResponseParameterName {
  ContentType,
  AccessControlAllowOrigin,
}

impl serde::Serialize for ResponseParameterName {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where S: serde::Serializer
  {
    format!("method.response.header.{}", self.to_string()).serialize(serializer)
  }
}
