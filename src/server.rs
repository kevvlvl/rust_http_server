use std::io::{Read};
use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::net::{TcpListener};
use crate::http::request::ParseError;

pub trait Handler {

    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request. {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {

    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {

            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request {}", String::from_utf8_lossy(&buffer));

                            let resp = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                },
                                Err(err) => {
                                    handler.handle_bad_request(&err)
                                }
                            };

                            if let Err(e) = resp.send(&mut stream) {
                                println!("Failed to send response {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection. {}", e),
                    }
                },

                Err(err) => println!("Failed to establish connection: {}", err),
            }

            let res = listener.accept();

            if res.is_err() {
                continue;
            }

            let (_, _) = res.unwrap();
        }
    }
}