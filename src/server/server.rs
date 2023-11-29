//orion api
mod orion_api;
mod json_types;
use reqwest::StatusCode;
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
use stargazer::liberror::orion_api_err::*;
//interprocess
use interprocess::os::windows::named_pipe::{self, tokio::PipeListenerOptionsExt};

use std::{
    ffi::OsStr,
    sync::Arc,
    borrow::Cow,
    convert::Infallible,
    net::SocketAddr
};

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, http};
use serde::Deserialize;

#[tokio::main]
pub async fn main() {
    
    let orion_api = Arc::new(OrionAPI::new());
    
    tokio::join!(http_server(orion_api.clone()),pipe_server(orion_api.clone()));
        
}


//http server
async fn http_server(orion_api: Arc<OrionAPI>)  {
   

    // Handler function for the web service
    async fn process_request(body: web::Bytes, orion_api: web::Data<Arc<OrionAPI>>) -> impl Responder {
        let request = String::from_utf8(body.to_vec()).unwrap();
        let response = handle_request(&request, orion_api.get_ref().clone()).await;
        HttpResponse::Ok().content_type("application/json").body(response)
    }

    println!("Starting up web server");
    // Start the HTTP server
    let _ = HttpServer::new(move || {
        App::new()
        .wrap(
            Cors::default()
                .allowed_origin("https://localhost:3000")
                .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
        )
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
                    //String must be a JSON string so extract error code
                    //TODO: Handle Errors
                    let resp = ResponseType::Login(LoginResponse {
                        status: StatusCode::UNAUTHORIZED.as_u16()
                    });
                    serde_json::to_string(&resp).unwrap()
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