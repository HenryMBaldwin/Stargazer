//orion api
mod orion_api;

//tokio
use tokio::net::TcpListener;

use futures::{AsyncReadExt, AsyncWriteExt};
use http_body_util::Full;
use orion_api::OrionAPI;
//shared pipe lib crate
use stargazer::libpipe::{
    consts,
    reqres::{RequestType, ResponseType, LoginResponse}
};

//interprocess
use interprocess::os::windows::named_pipe::{self, tokio::PipeListenerOptionsExt};

use std::{
    ffi::OsStr,
    sync::Arc,
    borrow::Cow,
    convert::Infallible,
    net::SocketAddr
};

//hyper
use hyper::{
    body::{self, Bytes, Body},
    server::conn::http1,
    service::service_fn,
    Request,
    Response,
};

use hyper_util::rt::TokioIo;

#[tokio::main]
pub async fn main() {
    
    let orion_api = Arc::new(OrionAPI::new());
    
        
        
}


//http server
async fn http_server(orion_api: Arc<OrionAPI>) {
    let addr = SocketAddr::from(([127,0,0,1],4200));
    let listener = TcpListener::bind(addr).await.expect("Error creating listener");

    //as anon function to capture orion_api
    let service = service_fn(move |request: Request<body::Incoming>| {
        let orion_api_clone = orion_api.clone();
        async move {
            // Extract the body as a string
            let whole_body = hyper::body::to_bytes(request.into_body()).await.expect("Failed to read body");
            let request_str = String::from_utf8(whole_body.to_vec()).expect("Body is not a valid UTF-8 string");
    
            // Call your handler function
            let response_body = handle_request(&request_str, orion_api_clone).await;
    
            // Create and send the response
            Ok::<_, Infallible>(Response::new(Full::<Bytes>::from(response_body)))
        }
    });

    loop {
        let (stream, _) = listener.accept().await.expect("Error accepting connection.");

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_http))
                .await
            {
                println!("Error serving the connection: {}", err);
            }
        });
    }
}
//pipe server
async fn pipe_server(orion_api: Arc<OrionAPI>) {
    //create PipeListener
    let listener: named_pipe::tokio::PipeListener<_> = named_pipe::PipeListenerOptions::new()
    .name(Cow::from(OsStr::new(consts::PIPE_NAME_SERVER)))
    .mode(named_pipe::PipeMode::Bytes)
    .create_tokio::<named_pipe::tokio::DuplexBytePipeStream>().expect("Error creating pipe");

    loop {
        
        

        let orion_api_clone = orion_api.clone();

        
        //blocks until connection is made
        let connection = listener.accept().await.expect("Error accepting connection");
        println!("Connection accepted");
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