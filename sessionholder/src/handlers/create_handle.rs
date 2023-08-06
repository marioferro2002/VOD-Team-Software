use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) fn handle_create(parts: &[&str], session_map: &Arc<Mutex<HashMap<String, String>>>) -> String {
    println!("Creating new key connection...");
    if parts.len() == 3 {
        let session_key = parts[0];
        let ip_port = format!("{}:{}", parts[1], parts[2]);
        let mut locked_map = session_map.lock().expect("Failed to acquire lock on session_map");
        locked_map.insert(session_key.to_string(), ip_port);
        println!("Key created");
        "CREATED".to_string()

    } else {
        println!("Not created key");
        "INVALID_CREATE".to_string()
    }
}