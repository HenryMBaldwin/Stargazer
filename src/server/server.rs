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

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[tokio::main]
pub async fn main() {
    
    let orion_api = Arc::new(OrionAPI::new());
    
    tokio::join!(http_server(orion_api.clone()),pipe_server(orion_api.clone()));
        
}


//http server
async fn http_server(orion_api: Arc<OrionAPI>)  {
    // Define a struct for incoming JSON requests
    #[derive(Deserialize)]
    struct JsonBody {
        request: String,
    }

    // Handler function for the web service
    async fn process_request(json_body: web::Json<JsonBody>, orion_api: web::Data<Arc<OrionAPI>>) -> impl Responder {
        let response = handle_request(&json_body.request, orion_api.get_ref().clone()).await;
        HttpResponse::Ok().content_type("application/json").body(response)
    }

    // Start the HTTP server
    let _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(orion_api.clone()))
            .route("/process", web::post().to(process_request))
    })
    .bind("127.0.0.1:4200").expect("error binding.")
    .run()
    .await;
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