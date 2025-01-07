use super::*;

pub struct Api<T>
where T: traits::Extensions + traits::ExtensionsApi
{
  inner: T,
}

/// Blanket implementation of the `Extend` trait for extensions applicable at API level.
impl<T> Api<T>
where T: traits::Extensions + traits::ExtensionsApi
{
  pub fn from(inner: T) -> Result<Api<T>> { Ok(Api { inner }) }

  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) -> Result<()> {
    let my_extensions = self.inner.extensions()?;
    match openapi.extensions.as_mut() {
      None => { openapi.extensions = Some(my_extensions); },
      Some(extensions) => { utils::merge_extensions(extensions, &my_extensions); },
    }
    Ok(())
  }
}

impl<T> utoipa::Modify for Api<T>
where T: traits::Extensions + traits::ExtensionsApi
{
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    Api::modify(self, openapi).unwrap();
  }
}

