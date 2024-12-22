use super::*;

#[derive(strum::Display)]
#[strum(serialize_all = "UPPERCASE")]
enum Source {
  Header,
  Authorizer,
}

pub fn header() -> ApiKeySource     { ApiKeySource { value: Source::Header, } }
pub fn authorizer() -> ApiKeySource { ApiKeySource { value: Source::Authorizer, } }

pub struct ApiKeySource {
  value: Source,
}

impl traits::Extensions for ApiKeySource {
  fn extensions(&self) -> Result<utoipa::openapi::extensions::Extensions> {
    Ok(utoipa::openapi::extensions::ExtensionsBuilder::new()
       .add("x-amazon-apigateway-api-key-source", self.value.to_string())
       .build())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use traits::Extensions;

  #[test]
  fn it_supports_header() -> Result<()> {
    let r = header();
    let extensions = r.extensions()?;
    assert!(
      extensions
      .get("x-amazon-apigateway-api-key-source").unwrap()
      .as_str().unwrap()
      .eq("HEADER")
    );

    Ok(())
  }

  #[test]
  fn it_supports_authorizer() -> Result<()> {
    let r = authorizer();
    let extensions = r.extensions()?;
    assert!(
      extensions
      .get("x-amazon-apigateway-api-key-source").unwrap()
      .as_str().unwrap()
      .eq("AUTHORIZER")
    );

    Ok(())
  }
}
