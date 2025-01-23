pub mod base;

use crate::communication::request::parse_request;
use std::net::TcpStream;

pub fn handle_connection(mut stream: TcpStream) {
    let (headers, body) = parse_request(&mut stream).expect("failed to extract parts");

    let parts: Vec<&str> = headers[0].split_whitespace().collect();

    let method = parts[0];
    let path = parts[1];

    match (method, path) {
        ("GET", "/") => base::handle_root(&mut stream),
        ("GET", "/hello") => base::handle_hello(&mut stream),
        _ if path.starts_with("/echo/") => {
            base::handle_echo(&mut stream, &path[6..]);
        }
        _ if path.starts_with("/user-agent") => {
            base::handle_user_agent(&mut stream, &headers[1..]);
        }
        _ if path.starts_with("/files") => {
            base::handle_file(&mut stream, &path[6..], body);
        }
        _ => base::handle_404(&mut stream),
    }
}
