use super::*;

mod named_request_validators; use named_request_validators::NamedRequestValidators;

pub fn from_name_validate_body_validate_parameters(name: &str, validate_body: bool, validate_parameters: bool) -> Result<RequestValidators> {
  Ok(
    RequestValidators {
      inner: named_request_validators::NamedRequestValidators {
        name: name.to_string(),
        request_validator: named_request_validators::RequestValidator {
          validate_request_body: validate_body,
          validate_request_parameters: validate_parameters,
        },
      },
    }
  )
}


pub struct RequestValidators {
  inner: NamedRequestValidators,
}

impl traits::Extensions for RequestValidators {
  fn extensions(&self) -> Result<utoipa::openapi::extensions::Extensions> {
    Ok(utoipa::openapi::extensions::ExtensionsBuilder::new()
      .add("x-amazon-apigateway-request-validators", serde_json::to_value(&self.inner)?)
      .build())
  }
}

impl traits::ExtensionsApi for RequestValidators {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_generates_extensions() -> Result<()> {
    use traits::Extensions;

    let extensions = from_name_validate_body_validate_parameters("my_extension", true, false)?.extensions()?;
    let my_extension = extensions
    .get("x-amazon-apigateway-request-validators").unwrap()
    .as_object().unwrap()
    .get("my_extension").unwrap();

    assert!(
      my_extension.as_object().unwrap()
      .get("validateRequestBody").unwrap()
      .as_bool().unwrap() == true
    );
    assert!(
      my_extension.as_object().unwrap()
      .get("validateRequestParameters").unwrap()
      .as_bool().unwrap() == false
    );
    Ok(())
  }
}
