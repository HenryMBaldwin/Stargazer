//manages both client instances and server instances
//There should only be one instance of both client and server
//The newest version of both client and server is favored
//In the case of multiple instances of the same version, the oldest instance is favored
//TODO: 
// - Because server and client versions should be in parity, the server, upon seeing a client that is newer than itself,
// gracefully kill itself so the client can start up a new server.

use chrono::Utc;
use stargazer::{libinstance::instance::{generate_id, ClientInstance, ServerInstance}, libpipe::{reqres::{RequestType, ResponseType, ServerNegotiationRequest}, consts::BUFFER_SIZE}};
use named_pipe::PipeClient;
use stargazer::libpipe::consts::PIPE_NAME;
use std::io::{Read, Write};

pub struct InstanceManager {
    client: Option<ClientInstance>,
    server: ServerInstance,
    active_queries: u32,
    ready_to_die: bool
}

impl InstanceManager {
    pub fn new() -> InstanceManager {

        let time = Utc::now();
        let id = generate_id();
        let version = env!("VERSION").to_string().parse::<u128>().expect("Unable to parse version");

        let server = ServerInstance {
            time,
            id,
            version,
        };

        InstanceManager {
            client: None,
            server,
            active_queries: 0,
            ready_to_die: false
        }
    }

    //incrememnt the active queries
    //this should really only be called when a query that should not be cancled is active
    pub fn add_query(&mut self) {
        self.active_queries += 1;
    }

    //decrement the active queries
    pub fn remove_query(&mut self) {
        self.active_queries -= 1;
    }

    //check if there are any active queries
    pub fn has_active_queries(&self) -> bool {
        if self.active_queries > 0 {
            return true;
        }
        return false;
    }

    //returns weather the server should die
    pub fn should_die(&self) -> bool {
        return self.ready_to_die;
    }

    //register a client instance
    //returns true if the client was registered
    //returns false if the client cannot be registered
    //a client that cannot be registered should kill itself
    //TODO: Logging
    pub fn register_client(&mut self, new_client: ClientInstance) -> bool {

        //check if the a is already registered
        match &self.client {
            Some(client) => {
                //check if its the same client
                if client.id == new_client.id {
                    return true;
                }
                else {
                    //favor the newest version
                    let current_version = client.version.parse::<u128>().expect(&format!("Unable to parse version {}", client.version));
                    let new_version = new_client.version.parse::<u128>().expect(&format!("Unable to parse version {}", new_client.version));
                    if new_version > current_version {
                        self.client = Some(new_client);
                        return true;
                    }
                    else if new_version < current_version {
                        return false;
                    }
                    else {
                        //if the versions are the same 
                        //favor the oldest instance
                        if new_client.time < client.time {
                            self.client = Some(new_client);
                            return true;
                        }
                        else {
                            return false;
                        }
                    }
                }
                
            }
            None => {
                self.client = Some(new_client);
                return true;
            }
        }
        

    } 

    // Determine if a the new server instance or this server instance should survive. 
    // Will need to implement atomic counter to keep track of open tasks.
    // returns true if the current server should kill itself
    pub fn contemplate_suicide(&self, new_server_version: u128) -> bool {
        //favor newest version
        let current_version = self.server.version;

        let new_version = new_server_version;

        if new_version > current_version {
            return true;
        }

        return false;
    }

    //Function to determine if another server instance and running, and which should stay alive
    //returns (hold, start)
    //returns (true, false) if this instance should hold and ask again
    //returns (false, true) if this instance should startup
    //returns (false, false) if this instance should kill itself
    pub fn confirm_startup(&self) -> (bool, bool) {
        //check and see if the pipe is open
        let mut client: PipeClient;

        match PipeClient::connect(PIPE_NAME) {
            Ok(cli) => {
                client = cli;
            },
            Err(_) => {
                return (false, true);
            }
        }

        //if pipe exists (another server exists) then negotiate with it
        let request = serde_json::to_string(&RequestType::ServerNegotiation(ServerNegotiationRequest{version: self.server.version.clone()})).expect("Error: error serializing json.");
        
        match client.write(request.as_bytes()) {
            Ok(_) => {},
            Err(_) => {
                return (false, true);
            }
        }

        let mut response = vec![0; BUFFER_SIZE];
        let size = client.read(&mut response);

        match size {
            Ok(size) => {
                let response = String::from_utf8(response[..size].to_vec());
                if response.is_err() {
                    return (false, true);
                }
                let response = response.unwrap();
                let response = serde_json::from_str::<ResponseType>(&response);
                if response.is_err() {
                    return (false, true);
                }
                let response = response.unwrap();
                match response {
                    ResponseType::ServerNegotiation(server_negotiation) => {
                        return (server_negotiation.hold, server_negotiation.start);
                    },
                    _ => {
                        return (false, true);
                    }
                }
            },
            Err(_) => {
                return (false, true);
            }
        }

        
    }

}

