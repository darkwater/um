use commandcodec::CommandCodec;
use futures::{Async, Future, Poll, Stream};
use response::Response;
use server::Server;
use std::sync::{Arc, Mutex};
use tokio::io;
use tokio::net::{TcpListener, TcpStream};

type State = Arc<Mutex<Server>>;

pub struct Client {
    stream: CommandCodec,
    state: State,
}

impl Client {
    pub fn new(socket: TcpStream, state: State) -> Self {
        // Wrap the socket with the `Lines` codec that we wrote above.
        let stream = CommandCodec::new(socket);

        Client {
            stream, state,
        }
    }
}

impl Future for Client {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        // Read new lines from the socket
        while let Async::Ready(cmd) = self.stream.poll()? {
            if cmd.is_none() {
                // EOF was reached. The remote client has disconnected.
                // There is nothing more to do.
                println!("client sent EOF");
                return Ok(Async::Ready(()));
            }
            let cmd = cmd.unwrap();

            println!("Received cmd: {:?}", cmd);

            match cmd {
                Ok(cmd) => {
                    let mut state = self.state.lock().unwrap();
                    let response = state.store.execute(cmd);
                    self.stream.buffer(response);
                },
                Err(e) => {
                    let response = Response::Error(e);
                    self.stream.buffer(response);
                },
            };

            self.stream.poll_flush()?;
        }

        // As always, it is important to not just return `NotReady`
        // without ensuring an inner future also returned `NotReady`.
        //
        // We know we got a `NotReady` from either `self.rx` or
        // `self.lines`, so the contract is respected.
        Ok(Async::NotReady)
    }
}
