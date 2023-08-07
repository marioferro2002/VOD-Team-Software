mod admin_auth;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];

    let auth_result = admin_auth::authenticate(&mut stream);
    match auth_result {
        Ok(admin_id) => {
            println!("Admin {} authenticated", admin_id);

            // Implement order distribution or other logic here

            // Example: Send a welcome message to the admin
            let welcome_message = "Welcome, Admin!";
            stream.write_all(welcome_message.as_bytes()).unwrap();
        }
        Err(_) => {
            println!("Authentication failed for admin");
            // Handle authentication failure
        }
    }

    stream.shutdown(std::net::Shutdown::Both).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:12345").expect("Failed to bind");
    println!("Server listening on 127.0.0.1:12345");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}