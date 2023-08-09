use std::io::{self, BufRead, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::thread;

pub(crate) fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:12345")?;

    println!("Enter the key:");
    let key = read_input()?;

    // Send the key to the server
    stream.write_all(key.as_bytes())?;

    // Wait for server response
    let mut reader = io::BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;

    // Check for AUTHENTICATED or error response
    if response.contains("AUTHENTICATED") {
        let private_code = extract_private_code(&response);
        if let Some(private_code) = private_code {
            println!("Received private code: {}", private_code);
        } else {
            println!("Invalid private code format.");
        }

        // Wait for 3 seconds
        thread::sleep(Duration::from_secs(3));
    } else if response.contains("error") {
        println!("Authentication failed.");
    } else {
        println!("Unexpected response: {}", response);
    }

    Ok(())
}

fn read_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn extract_private_code(response: &str) -> Option<u32> {
    let prefix = "private_code:";
    if let Some(start) = response.find(prefix) {
        let code_start = start + prefix.len();
        if let Some(end) = response[code_start..].find(' ') {
            if let Ok(private_code) = response[code_start..code_start + end].parse::<u32>() {
                return Some(private_code);
            }
        }
    }

    None
}
