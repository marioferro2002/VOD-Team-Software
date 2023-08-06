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

    let parts: Vec<&str> = received_data.trim().split(':').collect();
    if parts.is_empty() {
        return;
    }

    let response = match parts[0] {
        "JOIN" => handle_join(&parts[1], &session_map),
        "CREATE" => handle_create(&parts[1..], &session_map),
        _ => "INVALID_REQUEST".to_string(),
    };

    stream.write_all(response.as_bytes()).expect("Failed to write response");
}

fn handle_join(session_key: &str, session_map: &Arc<Mutex<HashMap<String, String>>>) -> String {
    println!("Joining with key connection...");
    let locked_map = session_map.lock().expect("Failed to acquire lock on session_map");
    if let Some(address_port) = locked_map.get(session_key) {
        format!("SESSION:{}:{}", session_key, address_port)
    } else {
        "INVALID_SESSION".to_string()
    }
}

fn handle_create(parts: &[&str], session_map: &Arc<Mutex<HashMap<String, String>>>) -> String {
    println!("Creating new key connection...");
    println!("The string length: {}", parts.len());
    thread::sleep(Duration::from_secs(2));
    if parts.len() == 3 {
        let session_key = parts[0];
        let ip_port = format!("{}:{}", parts[1], parts[2]);
        let mut locked_map = session_map.lock().expect("Failed to acquire lock on session_map");
        locked_map.insert(session_key.to_string(), ip_port);
        "CREATED".to_string()

    } else {
        println!("Not created key");
        "INVALID_CREATE".to_string()
    }
}