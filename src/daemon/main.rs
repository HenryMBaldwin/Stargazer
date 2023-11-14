
//orion api
mod orion_api;
mod reqres;


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
    const PIPE_NAME: &str = r"\\.\pipe\stargazer_pip";

    pub fn run() -> Result<()> {
        service_dispatcher::start(SERVICE_NAME, ffi_service_main)
    }

    define_windows_service!(ffi_service_main, my_service_main);

    pub fn my_service_main(_arguments: Vec<OsString>) {
        if let Err(_e) = run_service() {
            // Handle the error, by logging or something.
        }
    }

    pub fn run_service() -> Result<()> {
        let (shutdown_tx, shutdown_rx) = mpsc::channel();
        let mut runtime = tokio::runtime::Runtime::new().unwrap();
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
            let orion_api = Arc::new(OrionAPI::new());
            let mut pipe = File::create(PIPE_NAME).expect("Error: Failed to create named pipe.");
            
            loop {
                
                let mut buffer = [0; 1024];
                let pipe_clone = pipe.clone();
                match pipe_clone.read(&mut buffer) {
                    Ok(bytes_read) if bytes_read > 0 => {
                        let message = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                        let orion_api_clone = orion_api.clone();
        
                        tokio::spawn(async move {
                            let response = handle_request(&message, &orion_api_clone).await;
                            // Write response back to the pipe safely
                        });
                    }
                    Err(e) => {
                        //TODO: Handle Errors
                        ();
                    }
                    Ok(_) => {
                        //TODO: Figure out why I need this branch
                    }
                    
                }
        
                // Check for shutdown signal
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

    async fn handle_request(request: &str, orion_api: &OrionAPI) -> String {
        //Handle request to Orion API
        match serde_json::from_str::<RequestType>(request) {
            Ok(RequestType::Login(login_request)) => {
                String::new()
            }
            Err(e) =>   {
                //TODO Handle Error
                String::new()
            }
           }

    }
}
