use crate::ftprequests;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;

pub fn get_handler_client(req: &str, mut stream: &TcpStream) {
    if let Some(filename) = ftprequests::extract_filename(req) {
        let save_directory = "/home/fishe/active/dirs/cargoport-client/";

        let path = Path::new(save_directory).join(&filename);
        let response_filename = path.to_str().unwrap();

        ftprequests::get_handler(filename, &mut stream);

        let mut file = File::create(response_filename).expect("Failed to create file on client.");

        let mut buffer = [0; 4096];

        loop {
            match stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(bytes_read) => {
                    file.write_all(&buffer[..bytes_read])
                        .expect("Failed to write to file.");
                }
                Err(_) => {
                    let error_msg = "Error while receiving file from server.";
                    stream
                        .write_all(error_msg.as_bytes())
                        .expect("Failed to send error message.");
                }
            }
        }
        println!(
            "File '{}' recieved and saved to '{}'.",
            filename, response_filename,
        );
    } else {
        let error_msg = "That is not a valid filename.";
        stream
            .write_all(error_msg.as_bytes())
            .expect("Failed to write response");
    }
}
