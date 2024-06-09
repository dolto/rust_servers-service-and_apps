use std::{
    io::{Error, Read},
    net::TcpListener,
};

use http::httprequest::HttpRequest;

use crate::router::Router;

pub struct Server<'a> {
    socket_addr: &'a str,
}
impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }
    pub fn run(&self) -> Result<(), Error> {
        let connection_listener = TcpListener::bind(self.socket_addr)?;
        for stream in connection_listener.incoming() {
            let mut stream = stream?;
            println!("Connection established");
            let mut read_buffer = [0; 1028];
            stream.read(&mut read_buffer)?;
            let request_string = String::from_utf8(read_buffer.to_vec()).unwrap();
            let req: HttpRequest = request_string.into();
            Router::route(req, &mut stream);
        }
        Ok(())
    }
}
