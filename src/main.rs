use std::{io::Write, net::TcpListener};

fn main() {
    println!("http server!");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming(){

        println!("Connection established!");

        match stream {
            Ok(mut stream) =>{
                stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();

            },
            Err(err) => {

                eprintln!("{}", err);
            }
        }
    }
}
