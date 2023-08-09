use std::io::{self, Write, Read};
use std::net::TcpStream;

pub(crate) fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8081")?;

    println!("Enter the key:");
    let key = read_input()?;

    // Send the key to the server
    stream.write_all(key.as_bytes())?;

    // Wait for server response
    let mut buffer = [0u8; 1024];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
    let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Received: {}", received_data);

    // Check for "AUTHENTICATED" message and print the private code
    if received_data.contains("AUTHENTICATED") {
        if let Some(private_code) = extract_private_code(&received_data) {
            println!("Received private code: {}", private_code);
        }
    } else {
        println!("Authentication failed or unexpected response.");
    }
    loop {

    }
    Ok(())
}

fn read_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn extract_private_code(response: &str) -> Option<&str> {
    let prefix = "AUTHENTICATED, ";
    if let Some(start) = response.find(prefix) {
        let code_start = start + prefix.len();
        if let Some(end) = response[code_start..].find(']') {
            Some(&response[code_start..code_start + end])
        } else {
            None
        }
    } else {
        None
    }
}