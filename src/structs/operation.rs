use super::*;

pub mod request_validator;

pub fn from_path_method<E>(path: &str, http_method: utoipa::openapi::HttpMethod, inner: E) -> Operation<E> 
where E: traits::Extensions
{
  Operation {
    path: path.to_string(),
    http_method,
    inner,
  }
}

pub struct Operation<E>
where E: traits::Extensions
{
  path: String,
  http_method: utoipa::openapi::HttpMethod,
  inner: E,
}

impl<E> traits::ExtensionsOperation for Operation<E>
where E: traits::Extensions
{
  fn path(&self) -> &str { self.path.as_str() }
  fn http_method(&self) -> utoipa::openapi::HttpMethod { self.http_method.clone() }
  fn extensions(&self) -> Result<utoipa::openapi::extensions::Extensions> { self.inner.extensions() }
}

