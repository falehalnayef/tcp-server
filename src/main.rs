use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

fn main() {
    println!("Starting TCP server...");
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Listening on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");
                    handle_connection(stream);
            }
            Err(err) => {
                eprintln!("Failed to establish connection: {}", err);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("Handling connection from {}", peer_addr);

    let buf_reader = BufReader::new(&stream);
    let mut lines = buf_reader.lines();

    if let Some(Ok(request_line)) = lines.next() {
        println!("Request Line: {}", request_line);

        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 2 {
            eprintln!("Malformed request from {}", peer_addr);
            send_response(&mut stream, 400, "Bad Request", "Malformed request");
            return;
        }

        let method = parts[0];
        let path = parts[1];

        match (method, path) {
            ("GET", "/") => {
                send_response(&mut stream, 200, "OK", "Welcome to the server!");
            }
            ("GET", "/hello") => {
                send_response(&mut stream, 200, "OK", "Hello, world!");
            }
            _ => {
                send_response(&mut stream, 404, "Not Found", "The requested resource was not found.");
            }
        }
    } else {
        eprintln!("Failed to read request line from {}", peer_addr);
    }

    println!("Connection with {} closed", peer_addr);
}


fn send_response(stream: &mut TcpStream, status_code: u16, status_text: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        status_code,
        status_text,
        body.len(),
        body
    );
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send response: {}", e);
    }
}