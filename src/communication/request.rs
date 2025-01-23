use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

pub fn parse_request(stream: &mut TcpStream) -> Result<(Vec<String>, Option<String>), String> {
    let mut buf_reader = BufReader::new(stream);
    let mut headers = vec![];
    let mut body = None;

    let mut header_bytes = vec![];
    loop {
        let mut line = String::new();
        let bytes_read = buf_reader.read_line(&mut line).map_err(|e| format!("Failed to read line: {}", e))?;
        if bytes_read == 0 || line == "\r\n" {
            break;
        }
        header_bytes.extend_from_slice(line.as_bytes()); 
        headers.push(line.trim_end().to_string());
    }

    if let Some(content_length_line) = headers.iter().find(|line| line.starts_with("Content-Length:")) {
        if let Some(length_str) = content_length_line.split(": ").nth(1) {
            if let Ok(content_length) = length_str.parse::<usize>() {
                let mut body_content = vec![0; content_length];
                buf_reader.read_exact(&mut body_content).map_err(|e| format!("Failed to read body: {}", e))?;
                body = Some(String::from_utf8_lossy(&body_content).to_string());
            }
        }
    }

    Ok((headers, body))
}
