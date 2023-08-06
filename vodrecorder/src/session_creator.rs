use std::{env, thread};
use std::fs::File;
use std::io::{self, Write};
use std::net::TcpStream;
use std::time::Duration;
use ipconfig::get_adapters;
use chrono::{Local, Timelike};



pub(crate) fn create_session() {


    let adapters = get_adapters().expect("Failed to get network adapters"); // Get network adapters here
    //create the key
    let username = env::var("USERNAME").expect("Failed to get username");
    let now = Local::now().time(); // Get the time component
    let session_key = format!("{}{}{}{}", username, now.hour(), now.minute(), now.second());

    let mut local_ip = String::new();
    let port = 8082; // Port to listen on

    for adapter in adapters {
        if let Some(ip) = adapter.ip_addresses().iter().next() {
            local_ip = ip.to_string();
            break;
        }
    }

    if local_ip.is_empty() {
        println!("No IP addresses found for any adapter.");
        return;
    }

    let server_address = "127.0.0.1:8080"; // Replace with actual server IP

    let create_data = format!("CREATE:{}:{}:{}", session_key, local_ip, port);


    let mut stream = TcpStream::connect(server_address).expect("Failed to connect to server");
    stream
        .write_all(create_data.as_bytes())
        .expect("Failed to send data");

    println!("Session created. Key: {}", session_key);
    thread::sleep(Duration::from_secs(2));

}
