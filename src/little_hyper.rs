use crate::{Error, Logger, Request, Response, Result, Router};
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

/// Little Server
///
///
#[derive(Default)]
pub struct LittleServer {
    logger: Logger,
    router: Router,
}

impl LittleServer {
    /// Little Server
    ///
    /// New instance of `Little Server`
    pub fn new(log: bool) -> Self {
        Self {
            logger: Logger::new(log),
            ..Default::default()
        }
    }

    /// Add Router
    ///
    /// Add router to little-hyper router.
    pub fn add_router(&mut self, router: Router) {
        self.router.merge_router(router);
    }

    /// Handle Connection
    ///
    /// Handle the connection and send response.
    fn handle_connection(&self, stream: TcpStream) {
        let buffer = self.read_stream_info(&stream);

        if buffer.is_err() {
            // println!(">> Continued.");
            return;
        }

        let mut request = Request::from(buffer.unwrap());
        let mut response = Response::new(stream);

        self.logger.request(&request);

        if let Some(handler) = self.router.get_handler(&mut request) {
            handler(&mut request, &mut response);
        } else {
            // println!("ERROR: handler not found");
        }
    }

    /// Read Stream Info
    ///
    /// Read client info from TcpStream.
    fn read_stream_info(&self, stream: &TcpStream) -> Result<Vec<String>> {
        let buffer = BufReader::new(stream)
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect::<Vec<String>>();

        if buffer.is_empty() {
            return Err(Error::ReadStreamInfo);
        }

        Ok(buffer)
    }

    /// Listen
    ///
    /// Listen for incomming connections.
    pub fn listen<A: ToSocketAddrs>(&self, address: A) -> Result<()> {
        let listener = TcpListener::bind(address).expect("ERROR: Listen");

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.handle_connection(stream);
        }

        Ok(())
    }
}
