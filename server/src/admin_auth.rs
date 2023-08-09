use std::collections::HashSet;
use std::net::TcpStream;
use std::io::{Read, Write};
use rand::Rng;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

// Pretend key stored on the server
const ADMIN_KEY: &str = "dd6hh58d9ws9r7g6ghh56";

// Use a Mutex to track generated private codes
lazy_static! {
    static ref USED_PRIVATE_CODES: Arc<Mutex<HashSet<u32>>> = Arc::new(Mutex::new(HashSet::new()));
}

pub fn authenticate(stream: &mut TcpStream) -> Result<u32, ()> {
    let mut buffer = [0u8; 1024];

    // Read admin key from the client
    let bytes_read = stream.read(&mut buffer).unwrap();
    let admin_key = String::from_utf8_lossy(&buffer[..bytes_read]);

    if admin_key.trim() == ADMIN_KEY {
        let mut private_code: u32 = 0;

        // Generate a unique private code
        loop {
            private_code = rand::thread_rng().gen_range(1000..9999);
            let mut set = USED_PRIVATE_CODES.lock().unwrap();
            if !set.contains(&private_code) {
                set.insert(private_code);
                break;
            }
        }

        // Construct the response message with AUTHENTICATED and private code
        let response = format!("[AUTHENTICATED, {}]", private_code);
        println!("Sending response:{} ", response);
        stream.write_all(response.as_bytes()).expect("Failed to write response");
        stream.flush().unwrap();
        println!("Send response");

        // Authenticate the admin and return an admin ID
        Ok(private_code)  // Change this to an actual admin ID
    } else {
        // Key mismatch
        Err(())
    }
}