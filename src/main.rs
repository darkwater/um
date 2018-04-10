#![feature(match_default_bindings, nll, option_filter, try_trait)]
extern crate bytes;
#[macro_use]
extern crate futures;
extern crate tokio;

mod client;
mod command;
mod commandcodec;
mod node;
mod nodespec;
mod response;
mod server;
mod store;
mod value;

use server::Server;

fn main() {
    Server::new().run();
}
