use std::io;
mod admin;

fn main() {
    println!("Choose your role:");
    println!("1. Admin");
    println!("2. Client");

    let mut role = String::new();
    io::stdin().read_line(&mut role).expect("Failed to read line");
    let role = role.trim();

    match role {
        "1" => {
            println!("Running as admin...");
            admin::main();
        }
        "2" => {
            println!("Running as client...");
            // Implement client logic here
        }
        _ => {
            println!("Invalid choice: {}", role);
        }
    }
}