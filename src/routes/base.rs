use crate::communication::response::send_response;
use std::{
    env,
    fs::File,
    io::{self, Read},
    net::TcpStream,
    path::Path,
};

fn send_plain_text_response(
    stream: &mut TcpStream,
    status_code: u16,
    status_text: &str,
    message: &str,
) {
    send_response(stream, status_code, status_text, "text/plain", message);
}

fn get_directory() -> Option<String> {
    env::args()
        .position(|x| x == "--directory")
        .and_then(|index| env::args().nth(index + 1))
}

pub fn handle_root(stream: &mut TcpStream) {
    send_plain_text_response(stream, 200, "OK", "Welcome to the server!");
}

pub fn handle_404(stream: &mut TcpStream) {
    send_plain_text_response(
        stream,
        404,
        "Not Found",
        "The requested resource was not found.",
    );
}

pub fn handle_hello(stream: &mut TcpStream) {
    send_plain_text_response(stream, 200, "OK", "Hello, World!");
}

pub fn handle_echo(stream: &mut TcpStream, data: &str) {
    send_plain_text_response(stream, 200, "OK", data);
}

pub fn handle_user_agent(stream: &mut TcpStream, headers: &[String]) {
    if let Some(user_agent) = headers.iter().find_map(|val| {
        if val.starts_with("User-Agent") {
            Some(val.split(":").nth(1)?.trim())
        } else {
            None
        }
    }) {
        send_plain_text_response(stream, 200, "OK", user_agent);
    } else {
        handle_404(stream);
    }
}

pub fn handle_file(stream: &mut TcpStream, data: &str) {
    let dir = match get_directory() {
        Some(dir) => dir,
        None => {
            eprintln!("Error: No directory provided via --directory");
            handle_404(stream);
            return;
        }
    };

    let path = Path::new(&dir).join(data.trim_start_matches('/'));

    if !path.exists() {
        handle_404(stream);
        return;
    }

    let content_type = get_content_type(&path);

    match read_file(&path) {
        Ok(content) => send_response(stream, 200, "OK", content_type, &content),
        Err(_) => handle_404(stream),
    }
}

fn get_content_type(path: &Path) -> &str {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("txt") => "text/plain",
        Some("html") => "text/html",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        _ => "application/octet-stream",
    }
}

fn read_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    String::from_utf8(content)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 content"))
}
