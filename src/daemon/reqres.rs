use serde;
use reqwest::StatusCode;


//this file will define structs for calls/responses to/from OrionAPI through the named pipe
//this should be used by both client and server to ensure that communication is consistant on both ends

//JSON request should include a type field
//ex.
//
// {
//     "type": "Login",
//     "username": "username123",
//     "password": "pass123"
// } 



//types
#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum RequestType {
    Login(LoginRequest),
}

#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum ResponseType {
    Login(LoginResponse),
}

//func()

//struct request

//struct response


//login()
#[derive(serde::Deserialize)]
pub struct LoginRequest{
    username: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct LoginResponse{
    //where u16 is a status code, should always be checked
    //where String is an error
    response: Result<u16, String>
}

//query