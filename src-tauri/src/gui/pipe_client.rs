// Name: pipe_client.rs
// Description: provides simple functions that mirror orion_apis to simplify the process of calling through the named pipes.
// Important: Any functions in the orion_api in the server must be reflected here for the gui to have access to them.  

use std::io::{Write, Read};
use named_pipe::PipeClient;
use stargazer::libpipe::{reqres::{ RequestType, LoginRequest, ResponseType, CheckAuthRequest}, consts};
use reqwest::StatusCode;
use serde_json;


//takes in username and password to auth oapi
#[tauri::command]
pub async fn login(username: String, password: String) -> u16 {
    let request = serde_json::to_string(&RequestType::Login(LoginRequest {
        username: username,
        password: password
    })).expect("Error: error serializing json.");
    let response = serde_json::from_str::<ResponseType>(&send_wait(&request).await).expect("Error deserializing json.");
    match response {
        ResponseType::Login(login) => StatusCode::from_u16(login.status).expect("Error, invalid status code.").as_u16(),
        _ => {
            //TODO: Handle errors
            StatusCode::UNAUTHORIZED.as_u16()
        }
    }
    
}

//checks whether server has a valid login
#[tauri::command]
pub async fn check_auth() -> bool {
    let request = serde_json::to_string(&RequestType::CheckAuth(CheckAuthRequest{})).expect("Error: error serializing json.");
    let response = serde_json::from_str::<ResponseType>(&send_wait(&request).await).expect("Error deserializing json.");
    match response {
        ResponseType::CheckAuth(check_auth) => check_auth.result,
        _ => {
            //TODO: Handle errors
            false
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
