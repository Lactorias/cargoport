mod client;
mod ftprequests;

use std::io::{self, BufRead};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

fn handle_client(mut stream: TcpStream, dir: String, server_dir: String) {
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
                let mut data: Vec<u8> = Vec::new();
                if let Some(filename) = ftprequests::extract_filename(req) {
                    ftprequests::get_handler(filename, &mut stream, &mut data, &server_dir);
                    client::get_handler_client(req, &mut stream, &mut data, &dir);
                }
            }
            req if req.starts_with("DEL") => {
                println!("Del request recieved");
                if let Some(filename) = ftprequests::extract_filename(req) {
                    ftprequests::del_handler(filename, &mut stream, &server_dir);
                    println!("Successfully deleted the file: {}", filename);
                } else {
                    println!("Something went wrong...");
                }
            }
            req if req.starts_with("PUT") => {
                println!("Put request recieved");
                let mut data: Vec<u8> = Vec::new();
                if let Some(filename) = ftprequests::extract_filename(req) {
                    client::put_handler_client(filename, &mut stream, &mut data, &dir);
                    ftprequests::put_handler(filename, &mut stream, &mut data, &server_dir);
                }
            }
            req if req.starts_with("LIST_CLIENT") => {
                println!("List client request recieved");
                ftprequests::list_handler_client(&mut stream, &dir);
                println!("Successfully provided file list to user.");
            }
            req if req.starts_with("LIST") => {
                println!("List request recieved");
                ftprequests::list_handler(&mut stream, &server_dir);
                println!("Successfully provided file list to user.");
            }
            req if req.starts_with("QUIT") => {}
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
            Ok(mut stream) => {
                let welcome_message =
                    "Hello! Please enter the directory you wish to begin using.\n";
                stream
                    .write_all(welcome_message.as_bytes())
                    .expect("aaaaaaaaaaaaa");
                let mut buffer = [0; 1024];

                match stream.read(&mut buffer) {
                    Ok(bytes_read) => match String::from_utf8(buffer[..bytes_read].to_vec()) {
                        Ok(mut input) => {
                            input = input.trim().to_string();
                            println!("Clients directory: {}", input);

                            let server_prompt = "Please enter the server's directory.\n";
                            stream
                                .write_all(server_prompt.as_bytes())
                                .expect("Failed to write prompt.");

                            match stream.read(&mut buffer) {
                                Ok(bytes_read) => {
                                    match String::from_utf8(buffer[..bytes_read].to_vec()) {
                                        Ok(server_input) => {
                                            let server_dir = server_input.trim().to_string();
                                            println!("Server's directory, {}", server_dir);
                                            std::thread::spawn(|| {
                                                handle_client(stream, input, server_dir)
                                            });
                                        }
                                        Err(e) => {
                                            println!(
                                                "Failed to convert server directory input: {}",
                                                e
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Failed to read server directory from client: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed to convert bytes to string: {}", e);
                        }
                    },
                    Err(e) => {
                        println!("Failed to read directory from client: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}
