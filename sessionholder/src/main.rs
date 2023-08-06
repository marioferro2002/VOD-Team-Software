mod handlers;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind");
    println!("Server started. Waiting for connections...");

    let session_map: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let session_map_clone = Arc::clone(&session_map);
                thread::spawn(move || {
                    handle_client(stream, session_map_clone);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, session_map: Arc<Mutex<HashMap<String, String>>>) {
    let mut buffer = [0u8; 1024];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
    let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);

    let parts: Vec<&str> = received_data.trim().split(':').filter(|&s| !s.is_empty()).collect();
    println!("{:?}", parts);


    let response = match parts[0] {
        "JOIN" => handlers::join_handle::handle_join(&parts[1], &session_map),
        "CREATE" => handlers::create_handle::handle_create(&parts[1..], &session_map),
        _ => "INVALID_REQUEST".to_string(),
    };

    stream.write_all(response.as_bytes()).expect("Failed to write response");
}
