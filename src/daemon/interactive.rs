//orion api
mod orion_api;

use futures::{AsyncReadExt, AsyncWriteExt};
use orion_api::OrionAPI;
//shared pipe lib crate
use stargazer::libpipe::reqres::{self, RequestType, ResponseType, LoginResponse};

//interprocess
use interprocess::os::windows::named_pipe::{self, tokio::PipeListenerOptionsExt, PipeStream, PipeListener, DuplexMsgPipeStream};

//imports from main.rs, likely most will go unused as all windows service stuff has been stripped out
use std::{
    ffi::OsStr,
    io::{Read, Write},
    fs::File,
    sync::{mpsc, Arc},
    thread,
    time::Duration, borrow::Cow,
};
//use named_pipe::{PipeOptions, PipeClient, PipeServer, ConnectingServer};
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
#[tokio::main]
pub async fn main() -> Result<()> {
    
    let orion_api = Arc::new(OrionAPI::new());
    
        
        //create PipeListener
        let listener: named_pipe::tokio::PipeListener<_> = named_pipe::PipeListenerOptions::new()
            .name(Cow::from(OsStr::new(consts::PIPE_NAME_SERVER)))
            .mode(named_pipe::PipeMode::Bytes)
            .create_tokio::<named_pipe::tokio::DuplexBytePipeStream>().expect("Error creating pipe");

        loop {
            
            
           
             let orion_api_clone = orion_api.clone();

            //blocks until connection is made
            let connection = listener.accept().await.expect("Error accepting connection");
 
            tokio::spawn(async move {
                let (mut reader, mut writer) = connection.split();
                let mut buffer = vec![0; 1024];
                let read = reader.read(&mut buffer).await.expect("Error couldn't read string");
                let response = handle_request(&String::from_utf8_lossy(&buffer[..read]).to_string(), orion_api_clone).await;

                writer.write_all(response.as_bytes()).await.expect("Error Writing");
                writer.close().await.expect("Error closing the writer!");
                drop((reader, writer))
            });
    
        }
}


 

async fn handle_request(request: &str, orion_api: Arc<OrionAPI>) -> String{
    println!("Handling request: {}", request);
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