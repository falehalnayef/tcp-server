use std::net::TcpStream;
use crate::communication::response::send_response;

pub fn handle_root(stream: &mut TcpStream) {
    send_response(stream, 200, "OK", "Welcome to the server!");
}

pub fn handle_hello(stream: &mut TcpStream) {
    send_response(stream, 200, "OK", "Hello, World!");
}

