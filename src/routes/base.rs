use crate::communication::response::send_response;
use std::net::TcpStream;

pub fn handle_root(stream: &mut TcpStream) {
    send_response(stream, 200, "OK", "Welcome to the server!");
}

pub fn handle_hello(stream: &mut TcpStream) {
    send_response(stream, 200, "OK", "Hello, World!");
}

pub fn handle_echo(stream: &mut TcpStream, data: &str) {
    send_response(stream, 200, "OK", data);
}

pub fn handle_user_agent(stream: &mut TcpStream, data: &[String]) {
    for val in data {
        if val.starts_with("User-Agent") {
            send_response(
                stream,
                200,
                "OK",
                val.split(":")
                    .nth(1)
                    .expect("failed to access element")
                    .trim(),
            );
        }
    }
}
