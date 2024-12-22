use super::*;

pub struct XAmazonApigateway {
  openapi: utoipa::openapi::OpenApi,
}

impl XAmazonApigateway {
  pub fn from(openapi: utoipa::openapi::OpenApi) -> Self
  {
    XAmazonApigateway {
      openapi,
    }
  }

  pub fn extend<T, M>(self, e: &T) -> Result<Self>
  where T: traits::Extend<M>
  {
    Ok(XAmazonApigateway { 
      openapi: e.extend(self.openapi)?,
    })
  }

  pub fn build(self) -> utoipa::openapi::OpenApi {
    self.openapi
  }
}

