use super::*;

/// Struct wrapping extensions that applies at operation level defined by a path
/// and an http method.
pub struct Operation<T>
where T: traits::Extensions + traits::ExtensionsOperation
{
  path: String,
  http_method: utoipa::openapi::path::HttpMethod,
  inner: T,
}

impl<T> Operation<T>
where T: traits::Extensions + traits::ExtensionsOperation
{
  pub fn from_path_method(path: &str, http_method: utoipa::openapi::HttpMethod, inner: T) -> Result<Operation<T>>
  {
    Ok(
      Operation {
        path: path.to_string(),
        http_method,
        inner,
      }
    )
  }
}

/// Blanket implementation of the `Extend` trait for extensions applicable at operation level.
impl<T> Operation<T>
where T: traits::Extensions + traits::ExtensionsOperation 
{
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) -> Result<()> {
    if let Some(mut operation) = openapi.paths.get_path_operation(self.path.as_str(), self.http_method.clone()).cloned() {
      let my_extensions = self.inner.extensions()?;
      match operation.extensions.as_mut() {
        None => { operation.extensions = Some(my_extensions); },
        Some(extensions) => { utils::merge_extensions(extensions, &my_extensions); },
      };
      openapi.paths.add_path_operation(self.path.as_str(), vec![ self.http_method.clone() ], operation);
    } else {
      log::warn!("Operation for path [{verb}]{path} not found", path = self.path, 
        verb = match self.http_method {
          utoipa::openapi::path::HttpMethod::Get     => "GET",
          utoipa::openapi::path::HttpMethod::Post    => "POST",
          utoipa::openapi::path::HttpMethod::Put     => "PUT",
          utoipa::openapi::path::HttpMethod::Delete  => "DELETE",
          utoipa::openapi::path::HttpMethod::Options => "OPTIONS",
          utoipa::openapi::path::HttpMethod::Head    => "HEAD",
          utoipa::openapi::path::HttpMethod::Patch   => "PATCH",
          utoipa::openapi::path::HttpMethod::Trace   => "TRACE",
        },
      );
    }
    Ok(())
  }
}

impl<T> utoipa::Modify for Operation<T>
where T: traits::Extensions + traits::ExtensionsOperation
{
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    Operation::modify(self, openapi).unwrap();
  }
}
