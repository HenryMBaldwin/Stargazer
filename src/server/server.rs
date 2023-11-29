/// Name: server.rs
/// Description: This file is the main entry point for the server. It handles the creation of the named pipe and the http server and the proccessing of request.
/// Any functions that need to be exposed to either  must be added here and in the pipe_client.rs file in the gui.

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
    reqres::*
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
        //Login request
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
                    //Response must be a JSON string so extract error code
                    //TODO: Handle Errors
                    let resp = ResponseType::Login(LoginResponse {
                        status: StatusCode::UNAUTHORIZED.as_u16()
                    });
                    serde_json::to_string(&resp).unwrap()
                }
            }
        },
        //Query request
        Ok(RequestType::Query(query_request)) => {
            let result = orion_api.query(query_request.id, query_request.args).await;
            match result {
                Ok(data) =>  {
                    let resp = ResponseType::Query(QueryResponse {
                        status: StatusCode::OK.as_u16(),
                        result: data,
                    });
                    serde_json::to_string(&resp).unwrap()
                }
                Err(e) => {
                    let resp = ResponseType::Query(QueryResponse {
                        status: StatusCode::UNAUTHORIZED.as_u16(),
                        result: String::from(format!("Error: {}",e)),
                    });
                    serde_json::to_string(&resp).unwrap()
                }
            }
        },
        //GetQueryPrompts request
        Ok(RequestType::GetQueryPrompts(get_query_prompts_request)) => {
            let result = orion_api.get_query_prompts(get_query_prompts_request.id).await;
            match result {
                Ok(prompts) =>  {
                    let resp = ResponseType::GetQueryPrompts(GetQueryPromptsResponse {
                        status: StatusCode::OK.as_u16(),
                        prompts: prompts,
                    });
                    serde_json::to_string(&resp).unwrap()
                }
                Err(e) => {
                    let resp = ResponseType::GetQueryPrompts(GetQueryPromptsResponse {
                        status: StatusCode::UNAUTHORIZED.as_u16(),
                        prompts: String::from(format!("Error: {}",e)),
                    });
                    serde_json::to_string(&resp).unwrap()
                }
            }
        },
        //Error
        Err(e) => {
            //TODO Handle Error
            String::from(format!("Error: {}",e))
        }
    }
}