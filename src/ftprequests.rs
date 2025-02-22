use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::TcpStream;

pub fn get_handler(filename: &str, mut stream: &TcpStream, data: &mut Vec<u8>) {
    let ftp_root = std::env::var("FTP_ROOT")
        .unwrap_or_else(|_| String::from("/home/fishe/active/dirs/cargoport-test/"));
    let full_path = format!("{}/{}", ftp_root, filename);

    let mut file = match File::open(&full_path) {
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

    println!("File '{}' sent successfully!", filename);
}

pub fn del_handler(filename: &str, mut stream: &TcpStream) {
    let ftp_root = std::env::var("FTP_ROOT")
        .unwrap_or_else(|_| String::from("/home/fishe/active/dirs/cargoport-test/"));
    let full_path = format!("{}/{}", ftp_root, filename);

    match fs::remove_file(full_path) {
        Ok(()) => {
            let success_msg = "Succesfully deleted file!";
            stream
                .write_all(success_msg.as_bytes())
                .expect("Failed to alert of deletion.");
        }
        Err(_) => {
            let error_msg = "Failed to delete file.";
            stream
                .write_all(error_msg.as_bytes())
                .expect("Failed to alert of failed deletion.");
        }
    }
}

pub fn extract_filename(request: &str) -> Option<&str> {
    let mut parts = request.split_whitespace();
    if let (Some(command), Some(path)) = (parts.next(), parts.next()) {
        return Some(path);
    }
    None
}
