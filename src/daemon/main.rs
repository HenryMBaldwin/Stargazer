//orion api
mod orion_api;

//shared pipe lib crate
use stargazer::libpipe::reqres;


#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    stargazer_service::run()
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}

#[cfg(windows)]
mod stargazer_service {
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

    //Orion API
    use super::orion_api::OrionAPI;
    //Pipe request and response structures
    use super::reqres::*;

    const SERVICE_NAME: &str = "stargazer_service";
    const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;
    

    pub fn run() -> Result<()> {
        service_dispatcher::start(SERVICE_NAME, ffi_service_main)
    }

    define_windows_service!(ffi_service_main, my_service_main);

    pub fn my_service_main(_arguments: Vec<OsString>) {
        if let Err(_e) = run_service() {
            // TODO: Handle the error, by logging or something.
        }
    }

    pub fn run_service() -> Result<()> {
        let (shutdown_tx, shutdown_rx) = mpsc::channel();
        
        let orion_api = Arc::new(OrionAPI::new());
        let runtime = tokio::runtime::Runtime::new().unwrap();
        
        let (shutdown_signal, mut recv) = watch::channel(false);
        let event_handler = move |control_event| -> ServiceControlHandlerResult {
            match control_event {
                ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
                ServiceControl::Stop => {
                    shutdown_tx.send(()).unwrap();
                    ServiceControlHandlerResult::NoError
                }
                _ => ServiceControlHandlerResult::NotImplemented,
            }
        };

        let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Running,
            controls_accepted: ServiceControlAccept::STOP,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

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
        
                // Check for shutdown signal
                //TODO: fix this such that this isn't blocked while waiting for a client
                if *recv.borrow() {
                    break;
                }
            }
        });

        
        loop {
            if shutdown_rx.recv_timeout(Duration::from_secs(1)).is_ok() {
                let _ = shutdown_signal.send(true);
                break;
            }
        }

        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

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
}
