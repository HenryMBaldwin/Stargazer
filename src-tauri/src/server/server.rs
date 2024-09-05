#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Name: server.rs
/// Description: This file is the main entry point for the server. It handles the creation of the named pipe and the http server and the proccessing of request.
/// Any functions that need to be exposed to either  must be added here and in the pipe_client.rs file in the gui.


//mod
mod orion_api;
mod json_types;
mod cache_controller;
mod credential_manager;
mod logger;
mod query_tracker;
mod query_scheduler;
mod query_job_cache_controller;
mod instance_manager;

use futures::lock::Mutex;
use query_tracker::QueryTracker;
use reqwest::StatusCode;
use stargazer::libinstance::instance;
//tokio
use tokio::net::TcpListener;

use futures::{AsyncReadExt, AsyncWriteExt};
use http_body_util::Full;
use orion_api::OrionAPI;
use cache_controller::CacheController;
//shared pipe lib crate
use stargazer::libpipe::{
    consts,
    reqres::*
};
use stargazer::liberror::orion_api_err::*;

//instance manager
use instance_manager::InstanceManager;

//interprocess
use interprocess::os::windows::named_pipe::{self, tokio::PipeListenerOptionsExt};

use std::env;
use std::{
    ffi::OsStr,
    sync::Arc,
    borrow::Cow,
    convert::Infallible,
    net::SocketAddr
};
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, http};



#[tokio::main]
pub async fn main() {

    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "trace");

    //create instance manager
    let instance_manager = Arc::new(Mutex::new(InstanceManager::new()));
    //Check for other servers and negotiate

    //(hold, start)
    match instance_manager.lock().await.confirm_startup() {
        //holding overrides starting
        (true, _) => {
            //continually ask again every 5 seconds
            'negotiate: loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                match instance_manager.lock().await.confirm_startup() {
                    (true, _) => {
                        //do nothing
                    },
                    (false, true) => {
                        //start server
                        break 'negotiate;
                    },
                    (false, false) => {
                        //kill self
                        std::process::exit(0);
                    }
                }
            }
        },
        (_, true) => {
            //start server (do nothing)
        },
        (false, false) => {
            //kill self
            std::process::exit(0);
        }
    }
    let query_tracker = Arc::new(Mutex::new(query_tracker::QueryTracker::new()));
    let orion_api = Arc::new(OrionAPI::new(query_tracker.clone()).init().await.expect("Error creating OrionAPI"));
    let cache_controller = Arc::new(CacheController::new().expect("Error creating CacheController"));
    tokio::join!(http_server(orion_api.clone(), cache_controller.clone(), query_tracker.clone(), instance_manager.clone()), pipe_server(orion_api.clone(), cache_controller.clone(), query_tracker.clone(), instance_manager.clone()));
        
}


//http server
async fn http_server(orion_api: Arc<OrionAPI>, cache_controller: Arc<CacheController>, query_tracker: Arc<Mutex<QueryTracker>>, instance_manager: Arc<Mutex<InstanceManager>>) {
   

    // Handler function for the web service
    async fn process_request(body: web::Bytes, orion_api: web::Data<Arc<OrionAPI>>, cache_controller: web::Data<Arc<CacheController>>, query_tracker: web::Data<Arc<Mutex<QueryTracker>>> /* This is so cursed */, instance_manager: web::Data<Arc<Mutex<InstanceManager>>>) -> impl Responder {
        let request = String::from_utf8(body.to_vec()).unwrap();
        let response = handle_request(&request, orion_api.get_ref().clone(), cache_controller.get_ref().clone(), query_tracker.get_ref().clone(), instance_manager.get_ref().clone()).await;
        HttpResponse::Ok().content_type("application/json").body(response)

        //after every response check if the server should die
       //TODO: Implement this on http, for now it is only on pipe 
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
            .app_data(web::Data::new(cache_controller.clone()))
            .app_data(web::Data::new(query_tracker.clone()))
            .app_data(web::Data::new(instance_manager.clone()))
            .route("/process", web::post().to(process_request))
    })
    .bind("127.0.0.1:4200").expect("error binding.")
    .run()
    .await;
}

//pipe server
async fn pipe_server(orion_api: Arc<OrionAPI>, cache_controller: Arc<CacheController>, query_tracker: Arc<Mutex<QueryTracker>>, instance_manager: Arc<Mutex<InstanceManager>>) {
    //create PipeListener
    let listener: named_pipe::tokio::PipeListener<_> = named_pipe::PipeListenerOptions::new()
    .name(Cow::from(OsStr::new(consts::PIPE_NAME_SERVER)))
    .mode(named_pipe::PipeMode::Bytes)
    .create_tokio::<named_pipe::tokio::DuplexBytePipeStream>().expect("Error creating pipe");
    println!("Starting up pipe server");
    loop {
        let orion_api_clone = orion_api.clone();
        let db_controller_clone = cache_controller.clone();
        let query_tracker_clone = query_tracker.clone();
        let instance_manager_clone = instance_manager.clone();
        let instance_manager_clone2 = instance_manager.clone();
        //blocks until connection is made
        let connection = listener.accept().await.expect("Error accepting connection");
        println!("Pipe Server: Connection accepted");
        tokio::spawn(async move {
            let (mut reader, mut writer) = connection.split();
            let mut buffer = vec![0; consts::BUFFER_SIZE];
            let read = reader.read(&mut buffer).await.expect("Error couldn't read string");
            let response = handle_request(&String::from_utf8_lossy(&buffer[..read]).to_string(), orion_api_clone, db_controller_clone, query_tracker_clone, instance_manager_clone).await;

            writer.write_all(response.as_bytes()).await.expect("Error Writing");
            writer.close().await.expect("Error closing the writer!");
            drop((reader, writer));
            //check if server should die
            if instance_manager_clone2.lock().await.should_die() {
                std::process::exit(0);
            }
        });

    }
}

//handles requests, agnostic of whether they come from http or pipe server
async fn handle_request(request: &str, orion_api: Arc<OrionAPI>, cache_controller: Arc<CacheController>, query_tracker: Arc<Mutex<QueryTracker>>, instance_manager: Arc<Mutex<InstanceManager>>) -> String{
    println!("Server: Handling request: {}", request);
    //Handle request to Orion API
    match serde_json::from_str::<RequestType>(request) {
        //CheckAlive request
        Ok(RequestType::CheckAlive(_)) => {
            let resp = ResponseType::CheckAlive(CheckAliveResponse {
                status: true
            });
            serde_json::to_string(&resp).unwrap()
        },
        //Login request
        Ok(RequestType::Login(login_request)) => {
            //increment query count
            instance_manager.lock().await.add_query();
            let result = orion_api.login(login_request.username.as_str(), login_request.password.as_str()).await;
            match result {
                Ok(status) =>  {
                    let resp = ResponseType::Login(LoginResponse {
                        status: status.as_u16()
                    });
                    //decrement query count
                    instance_manager.lock().await.remove_query();
                    serde_json::to_string(&resp).unwrap()
                }
                Err(e) => {
                    //Response must be a JSON string so extract error code
                    //TODO: Handle Errors
                    let resp = ResponseType::Login(LoginResponse {
                        status: StatusCode::UNAUTHORIZED.as_u16()
                    });
                    instance_manager.lock().await.remove_query();
                    serde_json::to_string(&resp).unwrap()
                }
            }
        },
        //CheckAuth request
        Ok(RequestType::CheckAuth(_check_auth_request)) => {
            println!("Checking auth");
            let result = orion_api.check_auth().await;
            match result {
                true =>  {
                    println!("Auth success");
                    let resp = ResponseType::CheckAuth(CheckAuthResponse {
                        result: true,
                    });
                    serde_json::to_string(&resp).unwrap()
                }
                false => {
                    println!("Auth failed");
                    let resp = ResponseType::CheckAuth(CheckAuthResponse {
                        result: false,
                    });
                    serde_json::to_string(&resp).unwrap()
                }
            }
        },
        //Query request
        Ok(RequestType::Query(query_request)) => {
            //increment query count
            instance_manager.lock().await.add_query();

            let query_id: String = query_request.id.clone();
            let query_cache: bool = query_request.cache.clone();
            let query_args:  Vec<String> = query_request.args.clone();

            //check if query is cached
            if cache_controller.query_exists(query_id.clone(), &query_args).unwrap() && query_cache /* && false this disables caching for testinag purposes */ {
                let result = cache_controller.get_query(query_id.clone(), &query_args).unwrap();
                let resp = ResponseType::Query(QueryResponse {
                    status: StatusCode::OK.as_u16(),
                    result: result,
                });
                //decrement query count
                instance_manager.lock().await.remove_query();
                serde_json::to_string(&resp).unwrap()
            }
            else {
                let result = orion_api.query(query_id.clone(), query_args.clone()).await;
                match result {
                    Ok(data) =>  {
                        let resp = ResponseType::Query(QueryResponse {
                            status: StatusCode::OK.as_u16(),
                            result: data.clone(),
                        });
                        //cache query
                        cache_controller.insert_query(query_id, &query_args, &data).unwrap();
                        //decrement query count
                        instance_manager.lock().await.remove_query();
                        serde_json::to_string(&resp).unwrap()
                    }
                    Err(e) => {
                        let resp = ResponseType::Query(QueryResponse {
                            status: StatusCode::UNAUTHORIZED.as_u16(),
                            result: String::from(format!("Error: {}",e)),
                        });
                        //decrement query count
                        instance_manager.lock().await.remove_query();
                        serde_json::to_string(&resp).unwrap()
                    }
                }
            }
        },
        //GetQueryPrompts request
        Ok(RequestType::GetQueryPrompts(get_query_prompts_request)) => {
            //increment query count
            instance_manager.lock().await.add_query();
            let result = orion_api.get_query_prompts(get_query_prompts_request.id).await;
            match result {
                Ok(prompts) =>  {
                    let resp = ResponseType::GetQueryPrompts(GetQueryPromptsResponse {
                        status: StatusCode::OK.as_u16(),
                        prompts: prompts,
                    });
                    //decrement query count
                    instance_manager.lock().await.remove_query();
                    serde_json::to_string(&resp).unwrap()
                }
                Err(e) => {
                    let resp = ResponseType::GetQueryPrompts(GetQueryPromptsResponse {
                        status: StatusCode::UNAUTHORIZED.as_u16(),
                        prompts: String::from(format!("Error: {}",e)),
                    });
                    //decrement query count
                    instance_manager.lock().await.remove_query();
                    serde_json::to_string(&resp).unwrap()
                }
            }
        },
        //GetQueryLog request
        Ok(RequestType::GetQueryLog(get_query_log_request)) => {
            let result = query_tracker.lock().await.get_query_log();
            match result {
                Ok(log) =>  {
                    let resp = ResponseType::GetQueryLog(GetQueryLogResponse {
                        status: StatusCode::OK.as_u16(),
                        log: log,
                    });
                    serde_json::to_string(&resp).unwrap()
                }
                Err(e) => {
                    let resp = ResponseType::GetQueryLog(GetQueryLogResponse {
                        status: StatusCode::UNAUTHORIZED.as_u16(),
                        log: String::from(format!("Error: {}",e)),
                    });
                    serde_json::to_string(&resp).unwrap()
                }
            }
        },
        //ServerNegotiation request
        Ok(RequestType::ServerNegotiation(server_negotiation_request)) => {
            let result = instance_manager.lock().await.contemplate_suicide(server_negotiation_request.version);

            let mut hold = false;
            let mut start = false;

            match result {
                true => {
                    //ensure there are no active queries
                    if instance_manager.lock().await.has_active_queries() {
                        //tell the other server to hold
                        hold = true;
                    }
                    else {
                        //tell the other server to start
                        start = true;
                    }
                },
                false => {
                    //do nothing    
                }
            }
            //respond
            let resp = ResponseType::ServerNegotiation(ServerNegotiationResponse {
                status: StatusCode::OK.as_u16(),
                hold,
                start
            });
            serde_json::to_string(&resp).unwrap()
        },
        //Error
        Err(e) => {
            //TODO Handle Error
            String::from(format!("Error: {}",e))
        }
    }
}