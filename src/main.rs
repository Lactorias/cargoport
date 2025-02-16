mod client;
mod ftprequests;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = stream
            .read(&mut buffer)
            .expect("Failed to read from client.");

        if bytes_read == 0 {
            break;
        }

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Recieved request: {}", request);

        match request.as_ref() {
            req if req.starts_with("GET") => {
                println!("Get request recieved!");
                client::get_handler_client(req, &stream);
            }
            _ => {
                println!("Error, not a valid command!");
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address.");
    println!("Server Listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}
