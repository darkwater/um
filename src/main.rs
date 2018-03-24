#![feature(match_default_bindings, nll, try_trait)]
extern crate tokio;

mod command;
mod node;
mod nodespec;
mod response;
mod store;
mod value;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() {
    // Bind the server's socket
    let addr = "127.0.0.1:3535".parse().unwrap();
    let tcp = TcpListener::bind(&addr).unwrap();

    // Iterate incoming connections
    let server = tcp.incoming().for_each(|tcp| {
        // Copy the data back to the client
        let conn = io::write_all(tcp, "hlelo world\n")
            // print what happened
            .map(|res| {
                println!("{:?}", res)
            })
            // Handle any errors
            .map_err(|err| {
                println!("IO error {:?}", err)
            });

        // Spawn the future as a concurrent task
        tokio::spawn(conn);

        Ok(())
    })
    .map_err(|err| {
        println!("server error {:?}", err);
    });

    // Start the runtime and spin up the server
    tokio::run(server);
}
