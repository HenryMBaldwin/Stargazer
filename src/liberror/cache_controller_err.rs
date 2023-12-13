use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheControllerError{
    #[error("Key {0} has unsupported JSON type.")]
    UnsupportedJsonType(String),
    //generic error for create_table function. Possibly more specific errors will be added later    
    #[error("Couldn't create table for entry : {0}")]
    CreateTableError (String),
}
