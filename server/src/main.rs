mod admin_auth;

use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_authenticated_client(mut stream: TcpStream, _admin_id: u32) {
    // Logic to handle ongoing communication with authenticated client
    // You can implement your order distribution or other business logic here

    // Example: Continuously read messages from the client and send responses
    let mut buffer = [0u8; 1024];
    loop {
        match stream.read(&mut buffer) {
            // ... (rest of the code remains the same)
            Ok(_) => {}
            Err(_) => {}
        }
    }

    // Clean up: close the stream
    //stream.shutdown(std::net::Shutdown::Both).unwrap();
}

fn handle_client(mut stream: TcpStream) {
    let auth_result = admin_auth::authenticate(&mut stream);
    match auth_result {
        Ok(admin_id) => {
            println!("Admin {} authenticated", admin_id);
            loop {

            }
            //handle_authenticated_client(stream, admin_id);
        }
        Err(_) => {
            println!("Authentication failed for admin");
            // Handle authentication failure
        }
    }
}

fn main() {
    // Create the private_codes.txt file if it doesn't exist
    let private_codes_file = "private_codes.txt";
    if let Err(_e) = File::open(private_codes_file) {
        let mut file = File::create(private_codes_file).expect("Failed to create private_codes.txt");
        writeln!(file, "List of private codes received by admins:").expect("Failed to write to file");
    }

    let listener = TcpListener::bind("0.0.0.0:8081").expect("Failed to bind");
    println!("Server listening on 127.0.0.1:8081");

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