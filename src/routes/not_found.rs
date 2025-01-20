use crate::communication::response::send_response;
use std::net::TcpStream;

pub fn handle_404(stream: &mut TcpStream) {
    send_response(
        stream,
        404,
        "Not Found",
        "The requested resource was not found.",
    );
}
