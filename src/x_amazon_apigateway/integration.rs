use super::*;

mod ty; pub use ty::Type;
mod responses; pub use responses::Responses;
mod response; pub use response::Response;
mod response_parameters; pub use response_parameters::ResponseParameters;
mod request_parameters; pub use request_parameters::RequestParameters;

mod templates; 
pub type ResponseTemplates = templates::Templates;
pub type RequestTemplates = templates::Templates;


pub fn reference(name: &str) -> Result<Integration> {
  Ok(
    Integration {
      ty: Type::Ref(name.to_string()),
    }
  )
}

pub fn mock() -> Result<Integration> {
  mock_passthrough_request_templates_responses(None, None, None)
}

pub fn mock_passthrough_request_templates_responses(
  passthrough_behavior: Option<enums::PassthroughBehavior>,
  request_templates: Option<Vec<(&str, &str)>>,
  responses: Option<Vec<(&str, Response)>>,
) -> Result<Integration> {
  Ok(
    Integration {
      ty: Type::Definition(
        ty::IntegrationDefinition::Mock {
          passthrough_behavior: passthrough_behavior.unwrap_or(enums::PassthroughBehavior::Never),
          request_templates: request_templates.unwrap_or(vec![]).try_into()?,
          responses: responses.unwrap_or(vec![]).try_into()?,
        }
      ),
    }
  )
}

pub fn http_passthrough_uri_method_parameters_responses(
  passthrough_behavior: Option<enums::PassthroughBehavior>,
  uri: &str,
  method: utoipa::openapi::HttpMethod,
  parameters: Option<Vec<(&str, &str)>>,
  responses: Option<Vec<(&str, Response)>>,
) -> Result<Integration> {
  Ok(
    Integration {
      ty: Type::Definition(
        ty::IntegrationDefinition::Http {
          passthrough_behavior: passthrough_behavior.unwrap_or(enums::PassthroughBehavior::Never),
          uri: uri.to_string(),
          http_method: method,
          request_parameters: parameters.unwrap_or(vec![]).try_into()?,
          responses: responses.unwrap_or(vec![]).try_into()?,
        }
      )
    }
  )
}

pub struct Integration {
  ty: Type,
}

impl traits::Extensions for Integration {
  fn extensions(&self) -> Result<utoipa::openapi::extensions::Extensions> {
    Ok(utoipa::openapi::extensions::ExtensionsBuilder::new()
       .add("x-amazon-apigateway-integration", serde_json::to_value(&self.ty)?)
       .build())
  }
}
impl traits::ExtensionsOperation for Integration {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_makes_a_ref() -> Result<()> {
    use traits::Extensions;
    let extensions = reference("my_extension")?.extensions()?;
    assert!(
      extensions.get("x-amazon-apigateway-integration").unwrap()
      .as_object().unwrap()
      .get("$ref").unwrap().as_str().unwrap()
      .eq("#/components/x-amazon-apigateway-integrations/my_extension")
    );
    Ok(())
  }

  #[test]
  fn it_makes_a_mock() -> Result<()> {
    use traits::Extensions;
    let extensions = mock()?.extensions()?;
    assert!(
      extensions.get("x-amazon-apigateway-integration").unwrap()
      .as_object().unwrap()
      .get("type").unwrap().as_str().unwrap()
      .eq("mock")
    );
    Ok(())
  }
}
