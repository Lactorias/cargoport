use std::fs;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::net::TcpStream;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

pub fn get_handler(filename: &str, mut stream: &TcpStream, data: &mut Vec<u8>, ftp_root: &String) {
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
pub fn put_handler(filename: &str, mut stream: &TcpStream, data: &mut Vec<u8>, ftp_root: &String) {
    let full_path = format!("{}/{}", ftp_root, filename);

    let mut file = File::create(&full_path).expect("Failed to create a new file.");
    let mut cursor = Cursor::new(data);

    let mut buf = [0; 4096];

    loop {
        match cursor.read(&mut buf) {
            Ok(0) => break,
            Ok(bytes_read) => {
                file.write_all(&mut buf[..bytes_read])
                    .expect("Failed to write to file.\n");
            }
            Err(_) => {
                let error_msg = "Failed to read data from client file.";
                stream
                    .write_all(error_msg.as_bytes())
                    .expect("Failed to send error_msg.\n");
            }
        }
    }

    println!("File '{}' sent successfully!", filename);
}

pub fn del_handler(filename: &str, mut stream: &TcpStream, ftp_root: &String) {
    let full_path = format!("{}/{}", ftp_root, filename);

    match fs::remove_file(full_path) {
        Ok(()) => {
            let success_msg = "Succesfully deleted file!\n";
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

pub fn list_handler(stream: &mut TcpStream, ftp_root: &String) {
    let path = Path::new(ftp_root);

    match fs::read_dir(path) {
        Ok(elements) => {
            for element in elements {
                match element {
                    Ok(element) => {
                        let filename = element.file_name();
                        let filename_to_print = format!("{}\n", filename.to_string_lossy());
                        stream
                            .write_all(filename_to_print.as_bytes())
                            .expect("Could not send filename.");
                    }
                    Err(_) => {
                        let error_msg = "No such file present.";
                        stream
                            .write_all(error_msg.as_bytes())
                            .expect("Could not alert of no file.");
                    }
                }
            }
        }
        Err(_) => {
            let error_msg = "Failed to read from directory.";
            stream
                .write_all(error_msg.as_bytes())
                .expect("Failed to alert of unsuccesful read of directory.");
        }
    }
}

pub fn list_handler_client(stream: &mut TcpStream, dir: &String) {
    let path = Path::new(dir);

    match fs::read_dir(path) {
        Ok(elements) => {
            for element in elements {
                match element {
                    Ok(element) => {
                        let filename = element.file_name();
                        let filename_to_print = format!("{}\n", filename.to_string_lossy());
                        stream
                            .write_all(filename_to_print.as_bytes())
                            .expect("Could not send filename.");
                    }
                    Err(_) => {
                        let error_msg = "No such file present.";
                        stream
                            .write_all(error_msg.as_bytes())
                            .expect("Could not alert of no file.");
                    }
                }
            }
        }
        Err(_) => {
            let error_msg = "Failed to read from directory.";
            stream
                .write_all(error_msg.as_bytes())
                .expect("Failed to alert of unsuccesful read of directory.");
        }
    }
}

pub fn extract_filename(request: &str) -> Option<&str> {
    let mut parts = request.split_whitespace();
    if let (Some(_), Some(path)) = (parts.next(), parts.next()) {
        return Some(path);
    }
    None
}
