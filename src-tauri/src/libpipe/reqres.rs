use std::net::Shutdown;

/// Name: reqres.rs
/// Description: This file defines the structs that are used to communicate between the server and any clients.
/// Any functions that need to be exposed to either cleint must have request and response structs defined here.
/// 
use serde;
use serde::{Serialize, Deserialize};

use crate::libinstance::instance::ClientInstance;


//this file will define structs for calls/responses to/from OrionAPI through the named pipe
//this should be used by both client and server to ensure that communication is consistant on both ends

//JSON request should include a type field
//ex.
//
// {
//     "type": "Login",
//     "username": "username123",
//     "password": "pass123"
// } 



//types
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RequestType {
    CheckAlive(CheckAliveRequest),
    Login(LoginRequest),
    CheckAuth(CheckAuthRequest),
    Query(QueryRequest),
    GetQueryPrompts(GetQueryPromptsRequest),
    GetQueryLog(GetQueryLogRequest),
    ServerNegotiation(ServerNegotiationRequest),
    GetDatabases(GetDatabasesRequest),
    SwitchDatabase(SwitchDatabaseRequest),
    Logout(LogoutRequest),
    ShutdownServer(ShutdownServerRequest),
    GetServerVersion(GetServerVersionRequest),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseType {
    CheckAlive(CheckAliveResponse),
    Login(LoginResponse),
    CheckAuth(CheckAuthResponse),
    Query(QueryResponse),
    GetQueryPrompts(GetQueryPromptsResponse),
    GetQueryLog(GetQueryLogResponse),
    ServerNegotiation(ServerNegotiationResponse),
    GetDatabases(GetDatabasesResponse),
    SwitchDatabase(SwitchDatabaseResponse),
    Logout(LogoutResponse),
    ShutdownServer(ShutdownServerResponse),
    GetServerVersion(GetServerVersionResponse)
}


//struct request

//struct response

// //phone_home()
// #[derive(Serialize, Deserialize)]
// pub struct PhoneHomeRequest{
//     pub client: ClientInstance,
// }

// pub struct PhoneHomeResponse{
//     pub status: bool,
// }

//check_alive()
#[derive(Serialize, Deserialize)]
pub struct CheckAliveRequest{}

#[derive(Serialize, Deserialize)]
pub struct CheckAliveResponse{
    //obviously this will always be true if the server is alive
    pub status: bool
}

//login()
#[derive(Serialize, Deserialize)]
pub struct LoginRequest{
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse{
    //where u16 is a status code
    pub status: u16
}

//logout()
#[derive(Serialize, Deserialize)]
pub struct LogoutRequest{}

#[derive(Serialize, Deserialize)]
pub struct LogoutResponse{
    pub status: u16
}

//check_auth()
#[derive(Serialize, Deserialize)]
pub struct CheckAuthRequest{}

#[derive(Serialize, Deserialize)]
pub struct CheckAuthResponse{
    pub result: bool,
}


//query()
#[derive(Serialize, Deserialize)]
pub struct QueryRequest{
    pub id: String,
    pub cache: bool,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct QueryResponse{
    pub status: u16,
    pub result: String,
}

//get_query_prompts()
#[derive(Serialize, Deserialize)]
pub struct GetQueryPromptsRequest{
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetQueryPromptsResponse{
    pub status: u16,
    pub prompts: String,
}

//get_query_log()
#[derive(Serialize, Deserialize)]
pub struct GetQueryLogRequest{}

#[derive(Serialize, Deserialize)]
pub struct GetQueryLogResponse{
    pub status: u16,
    pub log: String,
}

//server negotation
#[derive(Serialize, Deserialize)]
pub struct ServerNegotiationRequest{
    pub version: u128,
}

//if hold is true, the server will wait for confirmation to start
//if start is true, it means the server should start after as the other server is about to die
//if both are false, the server should die
#[derive(Serialize, Deserialize)]
pub struct ServerNegotiationResponse{
    pub status: u16,
    pub hold: bool,
    pub start: bool,
}

//get_databases()
#[derive(Serialize, Deserialize)]
pub struct GetDatabasesRequest{}

#[derive(Serialize, Deserialize)]
pub struct GetDatabasesResponse{
    pub status: u16,
    pub databases: Vec<(String, String, bool)>,
}

//switch_database()
#[derive(Serialize, Deserialize)]
pub struct SwitchDatabaseRequest{
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SwitchDatabaseResponse{
    pub status: u16,
    pub databases: Vec<(String, String, bool)>,
}


//shutdown_server()
#[derive(Serialize, Deserialize)]
pub struct ShutdownServerRequest{}

#[derive(Serialize, Deserialize)]
pub struct ShutdownServerResponse{
    pub status: u16,
}

//get_server_version
#[derive(Serialize, Deserialize)]
pub struct GetServerVersionRequest{}

#[derive(Serialize, Deserialize)]
pub struct GetServerVersionResponse {
    pub version: String,
    pub status: u16,
}