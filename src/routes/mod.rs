pub mod base;
pub mod not_found;

use std::net::TcpStream;
use crate::communication::request::parse_request;

pub fn handle_connection(mut stream: TcpStream) {
    let request = parse_request(&mut stream);

    match request {
        Ok((method, path)) => {
            match (method.as_str(), path.as_str()) {
                ("GET", "/") => base::handle_root(&mut stream),
                ("GET", "/hello") => base::handle_hello(&mut stream),
                _ => not_found::handle_404(&mut stream),
            }
        }
        Err(e) => {
            eprintln!("Failed to parse request: {}", e);
        }
    }
}

