use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub(crate) fn join(ip: &str,port: &str ) {

    let server_address = "127.0.0.1:8082";
    let join_data = "hello";
    let mut stream = TcpStream::connect(server_address).expect("Failed to connect to server");
    thread::sleep(Duration::from_secs(2));
    stream
        .write_all(join_data.as_bytes())
        .expect("Failed to send data");

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .expect("Failed to read response");

    println!("{}", response);
}