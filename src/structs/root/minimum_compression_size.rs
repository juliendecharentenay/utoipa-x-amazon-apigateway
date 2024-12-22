use super::*;

pub fn from_size(size: usize) -> Result<MinimumCompressionSize> { 
  if size > 10_485_760 { return Err(error::Error::MinimumCompressionSizeError(size)); }
  Ok(MinimumCompressionSize { value: size }) 
}

pub struct MinimumCompressionSize {
  value: usize,
}

impl traits::Extensions for MinimumCompressionSize {
  fn extensions(&self) -> Result<utoipa::openapi::extensions::Extensions> {
    Ok(utoipa::openapi::extensions::ExtensionsBuilder::new()
       .add("x-amazon-apigateway-minimum-compression-size", self.value)
       .build())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use traits::Extensions;

  #[test]
  fn it_rejects_an_invalid_compression_size() -> Result<()> {
    assert!(from_size(14_000_000).is_err());
    Ok(())
  }

  #[test]
  fn it_supports_minimum_commpression_size() -> Result<()> {
    let extensions = from_size(12)?.extensions()?;
    assert!(extensions.get("x-amazon-apigateway-minimum-compression-size").unwrap()
       .as_u64().unwrap() == 12
    );
    Ok(())
  }
}

