pub mod base;
pub mod not_found;

use std::net::TcpStream;
use crate::communication::request::parse_request;

pub fn handle_connection(mut stream: TcpStream) {

    let request = parse_request(&mut stream).expect("failed to extract parts");

    let parts:Vec<&str> = request[0].split_whitespace().collect();

    let method = parts[0];
    let path = parts[1];

    match (method, path) {
                ("GET", "/") => base::handle_root(&mut stream),
                ("GET", "/hello") => base::handle_hello(&mut stream),
                _ if path.starts_with("/echo/") => {
                    let data: &str = &path[6..];
                    base::handle_echo(&mut stream, data);
                   }
                _ if path.starts_with("/user-agent") =>{
                    base::handle_user_agent(&mut stream, &request[1..]);
                } 
                _ => not_found::handle_404(&mut stream),
    }
}

