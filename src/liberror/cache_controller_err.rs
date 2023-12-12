use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheControllerError{
    #[error("Key {0} has unsupported JSON type.")]
    UnsupportedJsonType(String),
}