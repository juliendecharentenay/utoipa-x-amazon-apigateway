//! Library to augment `utoipa::openapi::OpenApi` with extensions compatible with 
//! [OpenAPI extensions for API Gateway](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions.html)
//!
//! To use, define your OpenAPI using utoipa as normal. Then extend the OpenAPI definition at run-time using the extensions defined
//! by this library:
//!
//! ```rust
//! use utoipa::{OpenApi, ToSchema};
//!
//! #[derive(ToSchema)]
//! struct Pet {
//!    id: u64,
//!    name: String,
//!    age: Option<i32>,
//! }
//!
//! struct NotFound {}
//! 
//! /// Get pet by id
//! ///
//! /// Get pet from database by pet id
//! #[utoipa::path(
//!     get,
//!     path = "/pets/{id}",
//!     responses(
//!         (status = 200, description = "Pet found successfully", body = Pet),
//!         (status = NOT_FOUND, description = "Pet was not found")
//!     ),
//!     params(
//!         ("id" = u64, Path, description = "Pet database id to get Pet for"),
//!     )
//! )]
//! async fn get_pet_by_id(pet_id: u64) -> Result<Pet, NotFound> {
//!     Ok(Pet {
//!         id: pet_id,
//!         age: None,
//!         name: "lightning".to_string(),
//!     })
//! }
//! 
//! #[derive(OpenApi)]
//! #[openapi(paths(get_pet_by_id))]
//! struct ApiDoc;
//!
//! // Extend with `x-amazon-apigateway-request-validators`
//! let openapi = ApiDoc::openapi();
//! let openapi = utoipa_x_amazon_apigateway::XAmazonApigateway::from(openapi)
//!   .extend(
//!    &utoipa_x_amazon_apigateway::request_validators::from_name_validate_body_validate_parameters(
//!      "body_validator", true, false
//!    )).unwrap()
//!   .extend(
//!    &utoipa_x_amazon_apigateway::request_validators::from_name_validate_body_validate_parameters(
//!      "parameter_validator", false, true
//!    )).unwrap()
//!   .build();
//! 
//! println!("{}", openapi.to_pretty_json().unwrap());
//!
//! let extensions = openapi.extensions.as_ref().unwrap();
//! assert!(extensions.len() == 1);
//!
//! let request_validators = extensions.get("x-amazon-apigateway-request-validators").unwrap()
//! .as_object().unwrap();
//! assert!(request_validators.contains_key("body_validator"));
//! assert!(request_validators.contains_key("parameter_validator"));
//!
//! let openapi = utoipa_x_amazon_apigateway::XAmazonApigateway::from(openapi)
//!   .extend(
//!      &utoipa_x_amazon_apigateway::operation::from_path_method(
//!        "/pets/{id}",
//!        utoipa::openapi::HttpMethod::Get,
//!        utoipa_x_amazon_apigateway::request_validator::from_name("body_validator"),
//!      )
//!    ).unwrap()
//!   .build();
//!
//! println!("{}", openapi.to_pretty_json().unwrap());
//!
//! assert!(
//!  openapi.paths
//!  .get_path_operation("/pets/{id}", utoipa::openapi::HttpMethod::Get).unwrap()
//!  .extensions.as_ref().unwrap()
//!  .get("x-amazon-apigateway-request-validator").unwrap()
//!  .as_str().unwrap().eq("body_validator")
//! );
//! 
//! ```

mod x_amazon_apigateway; pub use x_amazon_apigateway::XAmazonApigateway;

mod traits;
mod structs;
pub use structs::{
  root::{request_validators, api_key_source, },
  operation,
  operation::{request_validator,},
};
mod error; pub use error::Result;

