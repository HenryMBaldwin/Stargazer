//manages both client instances and server instances
//There should only be one instance of both client and server
//The newest version of both client and server is favored
//In the case of multiple instances of the same version, the oldest instance is favored
//TODO: 
// - Because server and client versions should be in parity, the server, upon seeing a client that is newer than itself,
// gracefully kill itself so the client can start up a new server.

use chrono::Utc;
use reqwest::Client;
use stargazer::libinstance::instance::{ClientInstance, ServerInstance, generate_id};

struct InstanceManager {
    client: Option<ClientInstance>,
    server: ServerInstance,
}

impl InstanceManager {
    pub fn new() -> InstanceManager {

        let time = Utc::now();
        let id = generate_id();
        let version = env!("VERSION").to_string();

        let server = ServerInstance {
            time,
            id,
            version,
        };

        InstanceManager {
            client: None,
            server,
        }
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
    pub fn contemplate_suicide(self, new_server: ServerInstance) -> bool {
        //favor newest version
        let current_version = self.server.version.parse::<u128>().expect(&format!("Unable to parse version {}", self.server.version));

        let new_version = new_server.version.parse::<u128>().expect(&format!("Unable to parse version {}", new_server.version));

        if new_version > current_version {
            return true;
        }

        return false;
    }

}

