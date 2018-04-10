use bytes::{BufMut, BytesMut};
use command::Command;
use futures::{Async, Poll, Stream};
use response::Response;
use std::fmt::Write;
use tokio::io::{self, AsyncRead, AsyncWrite};
use tokio::net::TcpStream;

pub struct CommandCodec {
    socket: TcpStream,
    rd: BytesMut,
    wr: BytesMut,
}

impl CommandCodec {
    /// Create a new `CommandCodec` backed by the socket
    pub fn new(socket: TcpStream) -> Self {
        CommandCodec {
            socket,
            rd: BytesMut::new(),
            wr: BytesMut::new(),
        }
    }

    pub fn buffer(&mut self, response: Response) {
        self.wr.extend_from_slice(format!("{}\n", response).as_bytes());
    }

    pub fn poll_flush(&mut self) -> Poll<(), io::Error> {
        // As long as there is buffered data to write, try to write it.
        while !self.wr.is_empty() {
            // Try to read some bytes from the socket
            let n = try_ready!(self.socket.poll_write(&self.wr));

            // As long as the wr is not empty, a successful write should
            // never write 0 bytes.
            assert!(n > 0);

            // This discards the first `n` bytes of the buffer.
            let _ = self.wr.split_to(n);
        }

        Ok(Async::Ready(()))
    }

    fn fill_read_buf(&mut self) -> Result<Async<()>, io::Error> {
        loop {
            // Ensure the read buffer has capacity.
            //
            // This might result in an internal allocation.
            self.rd.reserve(1024);

            // Read data into the buffer.
            //
           // The `read_buf` fn is provided by `AsyncRead`.
            let n = try_ready!(self.socket.read_buf(&mut self.rd));

            if n == 0 {
                return Ok(Async::Ready(()));
            }
        }
    }

    fn parse_command(line: &[u8]) -> Result<Command, &'static str> {
        let line = ::std::str::from_utf8(&line).map_err(|_| "invalid utf-8")?;
        let cmd  = line.parse()?;
        Ok(cmd)
    }
}

impl Stream for CommandCodec {
    type Item = Result<Command, &'static str>;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        // First, read any new data that might have been received
        // off the socket
        //
        // We track if the socket is closed here and will be used
        // to inform the return value below.
        let sock_closed = self.fill_read_buf()?.is_ready();

        // Now, try finding lines
        let pos = self.rd.iter().enumerate()
            .find(|&(_, bytes)| bytes == &b'\n')
            .map(|(i, _)| i);

        if let Some(pos) = pos {
            // Remove the line from the read buffer and set it
            // to `line`.
            let mut line = self.rd.split_to(pos + 2);

            // Drop the trailing \n
            line.split_off(pos);

            // Parse the bytes into a `Command`
            let cmd = Self::parse_command(&line);

            // Return the line
            return Ok(Async::Ready(Some(cmd)));
        }

        if sock_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}
