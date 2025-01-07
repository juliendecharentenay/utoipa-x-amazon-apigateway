use super::*;

/*
/// Trait that implements the logic that modify an openapi in place by adding
/// an extensions (or multiple) to a given OpenAPI.
/// A generic is used here to allow for blanket implementations for different situations.
pub trait Modify {
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) -> Result<()>;
}

/// Blanket implementation of `utoipa::Modify` trait
impl<T> utoipa::Modify for T
where T: Modify
{
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    Modify::modify(self, openapi).unwrap();
  }
}
*/

/// Trait to be implemented by any struct that provide a set of extensions
pub trait Extensions {
  fn extensions(&self) -> Result<utoipa::openapi::extensions::Extensions>;
}

/// Marker struct for extensions to be implemented at API level
pub trait ExtensionsApi {}

/// Marker struct for extensions to be implemented at Operation level
pub trait ExtensionsOperation {}

/*
/// Blanket implementation of the `Extend` trait for extensions applicable at root (API?) level.
impl<T> Modify for T
where T: Extensions + ExtensionsApi
{
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) -> Result<()> {
    let my_extensions = self.extensions()?;
    match openapi.extensions.as_mut() {
      None => { openapi.extensions = Some(my_extensions); },
      Some(extensions) => { utils::merge_extensions(extensions, &my_extensions); },
    }
    Ok(())
  }
}
*/


