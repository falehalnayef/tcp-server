use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn parse_request(stream: &mut TcpStream) -> Result<(String, String), String> {
    let buf_reader = BufReader::new(stream);
    let mut lines = buf_reader.lines();

    if let Some(Ok(request_line)) = lines.next() {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() >= 2 {
            let method = parts[0].to_string();
            let path = parts[1].to_string();
            return Ok((method, path));
        } else {
            return Err("Malformed request line".to_string());
        }
    }

    Err("Failed to read request".to_string())
}
