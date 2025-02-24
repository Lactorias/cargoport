use crate::ftprequests;
use std::fs::{remove_file, File};
use std::io::{Cursor, Read, Write};
use std::net::TcpStream;
use std::path::Path;

pub fn get_handler_client(
    req: &str,
    mut stream: &TcpStream,
    mut data: &Vec<u8>,
    save_directory: &String,
) {
    if let Some(filename) = ftprequests::extract_filename(req) {
        let path = Path::new(save_directory).join(&filename);
        let response_filename = path.to_str().unwrap();

        let mut file = File::create(response_filename).expect("Failed to create file on client.\n");
        let mut cursor = Cursor::new(data);

        let mut buffer = [0; 4096];

        loop {
            match cursor.read(&mut buffer) {
                Ok(0) => break,
                Ok(bytes_read) => {
                    println!("aaaaaaaaaaa");
                    file.write_all(&buffer[..bytes_read])
                        .expect("Failed to write to file.\n");
                }
                Err(_) => {
                    let error_msg = "Error while receiving file from server.\n";
                    stream
                        .write_all(error_msg.as_bytes())
                        .expect("Failed to send error message.\n");
                }
            }
        }
        println!(
            "File '{}' recieved and saved to '{}'.",
            filename, response_filename,
        );
    } else {
        let error_msg = "That is not a valid filename.\n";
        stream
            .write_all(error_msg.as_bytes())
            .expect("Failed to write response\n");
    }
}

pub fn put_handler_client(
    filename: &str,
    mut stream: &TcpStream,
    data: &mut Vec<u8>,
    curr_directory: &String,
) {
    let full_path = format!("{}/{}", curr_directory, filename);

    let mut file = match File::open(full_path) {
        Ok(f) => f,
        Err(_) => {
            let error_msg = "Error! This file is not present.";
            stream
                .write_all(error_msg.as_bytes())
                .expect("Failed to send error message.");
            return;
        }
    };

    let mut buf = [0; 4096];

    while let Ok(bytes_read) = file.read(&mut buf) {
        if bytes_read == 0 {
            break;
        }
        data.extend_from_slice(&buf[..bytes_read]);
    }

    let success = "Succesfully sent a request to place in server!.\n";
    stream
        .write_all(success.as_bytes())
        .expect("Failed to alert client of success");
}
