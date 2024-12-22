use super::*;

/// Utility function to merge extensions. utoipa provides a method which only look at the extension name level. So specify 
/// 2 extensions with the name will overwrite each other even if the extension is meant to be an array or object. This
/// prevent extension chaining.
fn merge_extensions(extensions: &mut utoipa::openapi::extensions::Extensions, other: &utoipa::openapi::extensions::Extensions) {
  for (key, value) in other.iter() {
    match extensions.get_mut(key) {
      Some(serde_json::value::Value::Null) 
      | Some(serde_json::value::Value::Bool(_))
      | Some(serde_json::value::Value::Number(_))
      | Some(serde_json::value::Value::String(_))
      | None 
      => { extensions.insert(key.clone(), value.clone()); },
      Some(serde_json::value::Value::Array(arr)) 
      => {
        if let serde_json::value::Value::Array(my_arr) = value {
          arr.append(&mut my_arr.clone());
        } else {
          log::warn!("Unable to merge Array with incompatible type");
        }
      },
      Some(serde_json::value::Value::Object(map)) 
      => {
        if let serde_json::value::Value::Object(my_map) = value {
          // Possible need to merge value here in place of overwriting.
          for (k,v) in my_map.iter() { map.insert(k.clone(), v.clone()); }
        } else {
          log::warn!("Unable to merge Object with incompatible type");
        }
      },
    }
  }
}

/// Trait that implements the logic to add an extensions (or multiple) to a given
/// OpenAPI.
/// A generic is used here to allow for blanket implementations for different situations.
pub trait Extend<Marker> {
  fn extend(&self, openapi: utoipa::openapi::OpenApi) -> Result<utoipa::openapi::OpenApi>;
}

/// Trait to be implemented by any struct that provide an (or multiple) extensions
pub trait Extensions {
  fn extensions(&self) -> Result<utoipa::openapi::extensions::Extensions>;
}

/// Marker struct for extensions to be implemented at root (API?) level
pub struct ExtensionsRootMarker;

/// Blanket implementation of the `Extend` trait for extensions applicable at root (API?) level.
impl<T> Extend<ExtensionsRootMarker> for T
where T: Extensions
{
  fn extend(&self, mut openapi: utoipa::openapi::OpenApi) -> Result<utoipa::openapi::OpenApi> {
    let my_extensions = self.extensions()?;
    match openapi.extensions.as_mut() {
      None => { openapi.extensions = Some(my_extensions); },
      Some(extensions) => { merge_extensions(extensions, &my_extensions); },
    }
    Ok(openapi)
  }
}

/// Trait to be implmented by any struct that provide extension(s) to be applied at operation level
/// (defined as path and http method).
pub trait ExtensionsOperation {
  fn path(&self) -> &str;
  fn http_method(&self) -> utoipa::openapi::path::HttpMethod;
  fn extensions(&self) -> Result<utoipa::openapi::extensions::Extensions>;
}

/// Marker struct for extensions to be implemented at operation (defined as path and http method) level
pub struct ExtensionsOperationMarker;

/// Blanket implementation of the `Extend` trait for extensions applicable at root (API?) level.
impl<T> Extend<ExtensionsOperationMarker> for T
where T: ExtensionsOperation 
{
  fn extend(&self, mut openapi: utoipa::openapi::OpenApi) -> Result<utoipa::openapi::OpenApi> {
    if let Some(mut operation) = openapi.paths.get_path_operation(self.path(), self.http_method()).cloned() {
      let my_extensions = self.extensions()?;
      match operation.extensions.as_mut() {
        None => { operation.extensions = Some(my_extensions); },
        Some(extensions) => { merge_extensions(extensions, &my_extensions); },
      };
      openapi.paths.add_path_operation(self.path(), vec![ self.http_method() ], operation);
    } else {
      log::warn!("Operation for path [{verb}]{path} not found", path = self.path(), 
        verb = match self.http_method() {
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
    Ok(openapi)
  }
}

