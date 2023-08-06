mod session_creator;

use std::env;
use std::io::{self, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use chrono::{Local, Timelike};


fn main() {
    println!("1. Create Session");
    println!("2. Join Session");

    let mut input = String::new();
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => {
            session_creator::create_session();
        }
        "2" => {
            join_session();
        }
        _ => {
            println!("Invalid choice");
        }
    }
}


fn join_session() {
    print!("Enter session key: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let session_key = input.trim();

    println!("Connecting to session with key: {}", session_key);
}