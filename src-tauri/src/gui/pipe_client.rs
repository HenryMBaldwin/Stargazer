// Name: pipe_client.rs
// Description: provides simple functions that mirror orion_apis to simplify the process of calling through the named pipes.
// Important: Any functions in the orion_api in the server must be reflected here for the gui to have access to them.  
// Important: Any functions that communicate with the server must ensure to call check_alive() before attempting to communicate with the server.
use std::env;
use stargazer::libpipe::reqres::GetQueryLogRequest;
use tokio::process::Command;
use std::io::{Read, Write};
use named_pipe::PipeClient;
use stargazer::libpipe::{reqres::{ RequestType, LoginRequest, ResponseType, CheckAuthRequest, CheckAliveRequest}, consts};
use reqwest::StatusCode;
use serde_json;
use anyhow::{Result,Error};

//checks if the server is alive and starts it if it isn't
#[tauri::command]
pub async fn check_alive() -> bool {
    let request = serde_json::to_string(&RequestType::CheckAlive(CheckAliveRequest{})).expect("Error: error serializing json.");
    //Should try to start server multiple times, this is jank though so should be revisited

    let send = &start_wait(&request).await;
    match send {
        Ok(_) => return true,
        Err(_) => {
            let status = start_server().await;
            match status {
                Ok(_) => return true,
                Err(_) => return false
            }
        }
    }
}

pub async fn start_server() -> Result<()> {
    let mut exe_path = env::current_exe().expect("Failed to get current executable path");
    exe_path.pop();
    let other_exe_path = exe_path.join("server.exe");
    
    Command::new(other_exe_path)
        .spawn()?;
    
    //Try 6 times to start it, waiting 5 seconds inbetween.
    //This seems to work alright should likely be revisited
    for _i in 0..6 {
        let send = &start_wait(&serde_json::to_string(&RequestType::CheckAlive(CheckAliveRequest{})).expect("Error: error serializing json")).await;
        match send {
            Ok(_) => {
                return Ok(());
            }
            Err(_) => {
                println!("Client: Server not started, waiting 5 seconds.");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }
    Err(Error::msg("Server could not be started"))
}
//takes in username and password to auth oapi
#[tauri::command]
pub async fn login(username: String, password: String) -> u16 {
    let request = serde_json::to_string(&RequestType::Login(LoginRequest {
        username: username,
        password: password
    })).expect("Error: error serializing json.");
    let response = serde_json::from_str::<ResponseType>(&send_wait(&request).await.expect("Error connecting to pipe")).expect("Error deserializing json.");
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
    check_alive().await;
    let request = serde_json::to_string(&RequestType::CheckAuth(CheckAuthRequest{})).expect("Error: error serializing json.");
    let response = serde_json::from_str::<ResponseType>(&send_wait(&request).await.expect("Error connecting to pipe")).expect("Error deserializing json.");
    match response {
        ResponseType::CheckAuth(check_auth) => check_auth.result,
        _ => {
            //TODO: Handle errors
            false
        }
    }
}

#[tauri::command]
pub async fn get_query_log() -> String {
    let request = serde_json::to_string(&RequestType::GetQueryLog(GetQueryLogRequest{})).expect("Error: error serializing json.");
    let response = serde_json::from_str::<ResponseType>(&send_wait(&request).await.expect("Error connecting to pipe")).expect("Error deserializing json.");
    match response {
        ResponseType::GetQueryLog(get_query_log) => get_query_log.log,
        _ => { String::from("{Error: failed to get query log.}") }
    }
}

// writes request to named pipe and waits for reponse, for check_alive()
async fn start_wait(request: &str) -> Result<String> {
    let mut client =  match PipeClient::connect(consts::PIPE_NAME) {
        Ok(client) => client,
        Err(e) => {
            return Err(Error::new(e));
        }
    };
    client.write(request.as_bytes()).expect("Error: client failed to write to pipe.");
    let mut response = vec![0; 1024];
    let size = client.read(&mut response).expect("Error: client failed to read from pipe.");
    Ok(String::from_utf8(response[..size].to_vec()).expect("Error converting response to string."))
}

// checks that server is alive, then writes request to named pipe and waits for reponse
async fn send_wait(request: &str) -> Result<String> {
    match check_alive().await {
        false => {
            return Err(Error::msg("Error: server could not be started"));
        }
        _ => {}
    }
    let mut client =  match PipeClient::connect(consts::PIPE_NAME) {
        Ok(client) => client,
        Err(e) => {
            return Err(Error::new(e));
        }
    };
    client.write(&request.as_bytes()).expect("Error: client failed to write to pipe.");
    let mut response = vec![0; 1024];
    let size = client.read(&mut response).expect("Error: client failed to read from pipe.");
    Ok(String::from_utf8(response[..size].to_vec()).expect("Error converting response to string."))
}
