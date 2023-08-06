use std::io::{self, Read, Write};
use std::net::TcpStream;
mod join_creator;


pub(crate) fn join() {

    println!("Enter the session key:");
    let mut session_key = String::new();
    io::stdin().read_line(&mut session_key).expect("Failed to read input");
    session_key = session_key.trim().to_string(); // Remove newline characters

    let server_address = "127.0.0.1:8080"; // Replace with the server's IP address and port
    let join_data = format!("JOIN:{}", session_key);

    let mut stream = TcpStream::connect(server_address).expect("Failed to connect to server");
    stream
        .write_all(join_data.as_bytes())
        .expect("Failed to send data");

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .expect("Failed to read response");

    let response_parts = response.trim().split(':').collect::<Vec<_>>();

    if response_parts.len() == 4 && response_parts[0] == "SESSION_RESPONSE" {
        let address = response_parts[2];
        let port = response_parts[3];


    } else {

        println!("Invalid session key or response format.");
    }

}
