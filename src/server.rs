use client::Client;
use std::sync::{Arc, Mutex};
use store::Store;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio;

pub struct Server {
    pub store: Store,
}

impl Server {
    pub fn new() -> Self {
        let store = Store::new();

        Server {
            store
        }
    }

    pub fn run(self) {
        // Bind the server's socket
        let addr = "127.0.0.1:3535".parse().unwrap();
        let tcp = TcpListener::bind(&addr).unwrap();

        let state = Arc::new(Mutex::new(self));

        // Iterate incoming connections
        let server = tcp.incoming().for_each(move |tcp| {
            Server::handle_connection(tcp, state.clone());

            Ok(())
        })
        .map_err(|err| {
            println!("server error {:?}", err);
        });

        // Start the runtime and spin up the server
        tokio::run(server);
    }

    pub fn handle_connection(socket: TcpStream, state: Arc<Mutex<Self>>) {
        let client = Client::new(socket, state)
            .map_err(|e| println!("error: {:#?}", e));

        // Spawn the future as a concurrent task
        tokio::spawn(client);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::io::prelude::*;
    use ::std::net::TcpStream;

    #[test]
    fn commands_over_tcp() {
    }
}
