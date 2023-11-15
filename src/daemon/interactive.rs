//orion api
mod orion_api;

use orion_api::OrionAPI;
//shared pipe lib crate
use stargazer::libpipe::reqres::{self, RequestType, ResponseType, LoginResponse};

//imports from main.rs, likely most will go unused as all windows service stuff has been stripped out
use std::{
    ffi::OsString,
    io::{Read, Write},
    fs::File,
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};
use named_pipe::{PipeOptions, PipeClient, PipeServer, ConnectingServer};
use stargazer::libpipe::consts;
use tokio::sync::{Notify, watch};
use windows_service::{ 
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher, Result,
};
//This serves as an interactive way to run the windows service logic without actually running the windows service, which cannot log and seems to be very hard to effectively debug at rutime.
pub fn main() -> Result<()> {
    
    let orion_api = Arc::new(OrionAPI::new());
    let runtime = tokio::runtime::Runtime::new().unwrap();
    


    //Named pipe logic in a different thread
    runtime.spawn(async move {
        
        
        
        loop {
            
            let server = PipeOptions::new(consts::PIPE_NAME)
            .single().unwrap(); //TODO: Handle Errors
            //blocks listening for an incoming connections
            let mut pipe_server = server.wait().unwrap();
            let orion_api_clone = orion_api.clone();

            tokio::spawn(async move {
                let mut buffer = vec![0; 1024];

                // Read request from client
                let bytes_read = pipe_server.read(&mut buffer).unwrap();
                let request = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

                // Process request and prepare response
                let response = handle_request(&request, orion_api_clone).await;

                // Write response back to client
                pipe_server.write(&response.as_bytes())
            });
    
        }
    });

    
    loop {
        // if shutdown_rx.recv_timeout(Duration::from_secs(1)).is_ok() {
        //     let _ = shutdown_signal.send(true);
        //     break;
        // }
        //necassary to similate the pipe logic happening on a different thread
        thread::sleep(Duration::MAX);
    }

    Ok(())
}

async fn handle_request(request: &str, orion_api: Arc<OrionAPI>) -> String{
    //Handle request to Orion API
    match serde_json::from_str::<RequestType>(request) {
        Ok(RequestType::Login(login_request)) => {
            let result = orion_api.login(login_request.username.as_str(), login_request.password.as_str()).await;
            match result {
                Ok(status) =>  {
                    let resp = ResponseType::Login(LoginResponse {
                        status: status.as_u16()
                    });
                    serde_json::to_string(&resp).unwrap()
                }
                Err(e) => {
                    //TODO: Handle Errors
                    String::from(format!("Error: {}", e))
                }
            }
        }
        //other request types
        Err(e) =>   {
            //TODO Handle Error
            String::from(format!("Error: {}",e))
        }
       }

}