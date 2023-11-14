use serde;
use serde::{Serialize, Deserialize};


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
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RequestType {
    Login(LoginRequest),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseType {
    Login(LoginResponse),
}

//func()

//struct request

//struct response


//login()
#[derive(Serialize, Deserialize)]
pub struct LoginRequest{
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse{
    //where u16 is a status code
    pub status: u16
}

//query