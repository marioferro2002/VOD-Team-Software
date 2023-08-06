use std::env;
use std::io::Write;
use std::net::TcpStream;
use ipconfig::get_adapters;
use chrono::{Local, Timelike};

mod connection_resolver;



pub(crate) fn create_session() {


    let adapters = get_adapters().expect("Failed to get network adapters"); // Get network adapters here
    //create the key
    let username = env::var("USERNAME").expect("Failed to get username");
    let now = Local::now().time(); // Get the time component
    let session_key = format!("{}{}{}{}", username, now.hour(), now.minute(), now.second());

    let mut local_ip = String::new();
    let port = 8082; // Port to listen on

    for adapter in adapters {
        for ip in adapter.ip_addresses() {
            if ip.is_ipv4() && !ip.is_loopback() && !ip.is_multicast() {
                local_ip = ip.to_string();
                println!("the ip is: {}", local_ip);
                break;
            }
        }
    }

    if local_ip.is_empty() {
        println!("No IP addresses found for any adapter.");
        return;
    }

    let server_address = "127.0.0.1:8080"; // Replace with actual server IP

    let create_data = format!("CREATE:{}:{}:{}", session_key, local_ip, port);

    println!("Sending: {}", create_data);
    let mut stream = TcpStream::connect(server_address).expect("Failed to connect to server");
    stream
        .write_all(create_data.as_bytes())
        .expect("Failed to send data");

    println!("Session created. Key: {}", session_key);

    //starts creator panel
    connection_resolver::main();


}
