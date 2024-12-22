use super::*;

use serde::ser::SerializeMap;

pub fn from_name_validate_body_validate_parameters(name: &str, validate_body: bool, validate_parameters: bool) -> RequestValidators {
  RequestValidators {
    inner: NamedRequestValidators {
      name: name.to_string(),
      request_validator: RequestValidator {
        validate_request_body: validate_body,
        validate_request_parameters: validate_parameters,
      },
    },
  }
}

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
    let mut s = serializer.serialize_map(Some(1))?;
    s.serialize_entry(
      &self.name,
      &self.request_validator,
    )?;
    s.end()
  }
}

pub struct RequestValidators {
  pub inner: NamedRequestValidators,
}

/*
impl serde::Serialize for RequestValidators {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where S: serde::Serializer
  {
    let mut s = serializer.serialize_map(Some(1))?;
    s.serialize_entry(
      "x-amazon-apigateway-request-validators",
      &self.inner,
    )?;
    s.end()
  }
}
*/

impl traits::Extensions for RequestValidators {
  fn extensions(&self) -> Result<utoipa::openapi::extensions::Extensions> {
    Ok(utoipa::openapi::extensions::ExtensionsBuilder::new()
      .add("x-amazon-apigateway-request-validators", serde_json::to_value(&self.inner)?)
      .build())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_serializes() -> Result<()> {
    let i = NamedRequestValidators {
        name: "body".to_string(),
        request_validator: RequestValidator {
          validate_request_body: true,
          validate_request_parameters: false,
        },
    };
    let v: serde_json::Value = serde_json::to_value(&i)?;
    println!("{v:#?}");

    assert!(
      v.as_object().unwrap()
      .get("body").unwrap()
      .as_object().unwrap()
      .get("validateRequestBody").unwrap()
      .as_bool().unwrap() == true);
    Ok(())
  }
}



