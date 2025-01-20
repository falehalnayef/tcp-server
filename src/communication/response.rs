use std::io::Write;
use std::net::TcpStream;

pub fn send_response(stream: &mut TcpStream, status_code: u16, status_text: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        status_text,
        body.len(),
        body
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send response: {}", e);
    }
}
