
pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Minimum compression size is to be an integer between 0 and 10485760. Provided: `{0}`")]
  MinimumCompressionSizeError(usize),
  #[error(transparent)]
  SerdeJsonError(#[from] serde_json::Error),
}
