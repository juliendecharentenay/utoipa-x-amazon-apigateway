use super::*;

pub enum Type {
  Ref(String),
  Definition(IntegrationDefinition),
}

impl serde::Serialize for Type {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where S: serde::Serializer
  {
    use serde::ser::SerializeMap;
    match self {
      Type::Ref(v) => utils::serialize_ref(serializer, format!("#/components/x-amazon-apigateway-integrations/{v}")),
      Type::Definition(IntegrationDefinition::Mock {
        passthrough_behavior,
        request_templates,
        responses,
      }) => {
        let mut s = serializer.serialize_map(Some(4))?;
        s.serialize_entry("type", "mock")?;
        s.serialize_entry("passthroughBehavior", &passthrough_behavior.to_string())?;
        s.serialize_entry("requestTemplates", request_templates)?;
        s.serialize_entry("responses", responses)?;
        s.end()
      },
      Type::Definition(IntegrationDefinition::Http {
        passthrough_behavior, uri, http_method, request_parameters, responses
      }) => {
        let mut s = serializer.serialize_map(Some(6))?;
        s.serialize_entry("type", "http")?;
        s.serialize_entry("passthroughBehavior", &passthrough_behavior.to_string())?;
        s.serialize_entry("uri", uri)?;
        s.serialize_entry("httpMethod", http_method)?;
        s.serialize_entry("requestParameters", request_parameters)?;
        s.serialize_entry("responses", responses)?;
        s.end()
      },
    }
  }
}

/// Integration defintions
pub enum IntegrationDefinition {
  Mock {
    passthrough_behavior: enums::PassthroughBehavior,
    request_templates: RequestTemplates,
    responses: Responses,
  },
  /* TODO
  AwsProxy {
  },
  Aws {
  },
  */
  Http {
    passthrough_behavior: enums::PassthroughBehavior,
    uri: String,
    http_method: utoipa::openapi::HttpMethod,
    request_parameters: RequestParameters,
    responses: Responses,
  },
  /*
  HttpProxy {
  },
  */
}


