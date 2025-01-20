mod communication;
mod config;
mod routes;

use config::get_server_address;
use routes::handle_connection;
use std::{net::TcpListener, thread};

fn main() {
    let address = get_server_address();

    println!("Starting server at {}", address);
    let listener = TcpListener::bind(&address).expect("Failed to bind to address");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
}
