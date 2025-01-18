use std::net::TcpStream;
use crate::communication::response::send_response;

pub fn handle_404(stream: &mut TcpStream) {
    send_response(stream, 404, "Not Found", "The requested resource was not found.");
}

