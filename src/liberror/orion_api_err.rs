use thiserror::Error;
use reqwest::StatusCode;
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Password is empty.")]
    EmptyPassword,
    #[error("Username is empty.")]
    EmptyUsername,
    #[error("Login rejected by Orion. Status Code: {0}")]
    InavalidLogin(StatusCode),
    #[error("Auth is invalid for unknown reasons: {0}")]
    Unknown(String)
}

#[derive(Error, Debug)]
pub enum QueryError{
    #[error("Unknown query error")]
    Unknown
}