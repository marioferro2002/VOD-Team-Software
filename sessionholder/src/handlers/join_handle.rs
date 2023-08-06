use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub(crate) fn handle_join(session_key: &str, session_map: &Arc<Mutex<HashMap<String, String>>>) -> String {
    println!("Joining with key connection...");
    let locked_map = session_map.lock().expect("Failed to acquire lock on session_map");
    if let Some(address_port) = locked_map.get(session_key) {
        println!("Sending join response...");
        thread::sleep(Duration::from_secs(2));
        format!("SESSION_RESPONSE:{}:{}",session_key ,address_port ) // Modify port as needed
    } else {
        "INVALID_SESSION".to_string()
    }
}