use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

pub(crate) fn main() {
    let listener = TcpListener::bind("127.0.0.1:8082").expect("Failed to bind");

    println!("Listening on port 8082...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
    let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Received: {}", received_data);

    let response = "Hello from the server!";
    stream.write_all(response.as_bytes()).expect("Failed to write response");
    thread::sleep(Duration::from_secs(2));
}