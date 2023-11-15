use std::io::{Write, Read};

use named_pipe::PipeClient;
//provides simple functions that mirror orion_apis to simplify the process of calling through the named pipes
use stargazer::libpipe::{reqres::{ RequestType, LoginRequest, ResponseType}, consts};
use reqwest::StatusCode;
use serde_json;
// let client = PipeClient::connect(consts::PIPE_NAME).expect("Error: Error creating pipe client with given name");
//     client.write(serde_json::to_string(&RequestType::Login(LoginRequest { 
//         username: username,
//         password: password 
//     })).unwrap().as_bytes());

//     let mut buffer = vec![0; 1024];
//     client.read(&mut buffer);
//     buffer

//takes in username and password to auth oapi
pub async fn login(username: &str, password: &str) -> StatusCode {
    let request = serde_json::to_string(&RequestType::Login(LoginRequest {
        username: String::from(username),
        password: String::from(password)
    })).expect("Error: error serializing json.");
    let response = serde_json::from_str::<ResponseType>(&send_wait(&request).await).expect("Error deserializing json.");
    match response {
        ResponseType::Login(login) => StatusCode::from_u16(login.status).expect("Error, invalid status code."),
        _ => {
            //TODO: Handle errors
            StatusCode::UNAUTHORIZED
        }
    }
    
}

// writes request to named pipe and waits for reponse
async fn send_wait(request: &str) -> String {
    let mut client = PipeClient::connect(consts::PIPE_NAME).expect("Error: Error creating pipe client with given name.");
    client.write(request.as_bytes()).expect("Error: client failed to write to pipe.");
    let mut response = vec![0; 1024];
    let size = client.read(&mut response).expect("Error: client failed to read from pipe.");
    String::from_utf8(response[..size].to_vec()).expect("Error converting response to string.")
}
