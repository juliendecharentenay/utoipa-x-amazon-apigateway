use super::*;

/// Pendent to [integration.response](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-integration-response.html)
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
  status_code: String,
  response_templates: Option<ResponseTemplates>,
  response_parameters: ResponseParameters,
}

impl Response {
  pub fn from_status_code_parameters<P>(
    status_code: &str,
    response_parameters: P,
  ) -> Result<Response>
  where P: std::convert::TryInto<ResponseParameters, Error=error::Error>,
  {
    Ok(
      Response {
        status_code: status_code.to_string(), 
        response_templates: None,
        response_parameters: response_parameters.try_into()?, 
      }
    )
  }

  pub fn from_status_code_templates_parameters<T, P>(
    status_code: &str,
    response_templates: T,
    response_parameters: P,
  ) -> Result<Response> 
  where T: std::convert::TryInto<ResponseTemplates, Error=error::Error>,
        P: std::convert::TryInto<ResponseParameters, Error=error::Error>,
  {
    Ok( 
      Response { 
        status_code: status_code.to_string(), 
        response_templates: Some(response_templates.try_into()?), 
        response_parameters: response_parameters.try_into()?, 
      } 
    )
  }
}
