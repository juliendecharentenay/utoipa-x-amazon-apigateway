/// Macro to make a modifier that applies at API level
#[macro_export]
macro_rules! define_api_modifier {
  ( $i:ident, $expression:expr ) => {
    pub struct $i;
    impl utoipa::Modify for $i {
      fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        utoipa_x_amazon_apigateway::extensions::Api::from(
          $expression.unwrap()
        ).unwrap().modify(openapi);
      }
    }
  };
}

/// Macro to make a modifier that applies at Operation level
#[macro_export]
macro_rules! define_operation_modifier {
  ( $i:ident, $path:literal, $http_method:path, $expression:expr ) => {
    pub struct $i;
    impl utoipa::Modify for $i {
      fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        utoipa_x_amazon_apigateway::extensions::Operation::from_path_method(
          $path,
          $http_method,
          $expression.unwrap(),
        ).unwrap().modify(openapi);
      }
    }
  };
}
          
