use super::*;

pub fn from_name(name: &str) -> RequestValidator {
  RequestValidator { name: name.to_string() }
}

pub struct RequestValidator {
  name: String,
}

impl traits::Extensions for RequestValidator {
  fn extensions(&self) -> Result<utoipa::openapi::extensions::Extensions> {
    Ok(utoipa::openapi::extensions::ExtensionsBuilder::new()
       .add("x-amazon-apigateway-request-validator", serde_json::to_value(&self.name)?)
       .build())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_makes_extensions() -> Result<()> {
    use traits::Extensions;

    let request_validator = from_name("body");
    let extensions = request_validator.extensions()?;
    assert!(
      extensions.get("x-amazon-apigateway-request-validator").unwrap()
      .as_str().unwrap().eq("body")
    );
    Ok(())
  }
}

