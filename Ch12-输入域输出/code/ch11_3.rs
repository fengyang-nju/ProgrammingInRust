use std::io;

fn main() {
    let mut name = String::new();
    let mut age = String::new();

    println!(">>Please enter your name:");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!(">>Please enter your age:");
    io::stdin().read_line(&mut age).expect("Failed to read line");

    println!("Hello, {}! You are {} years old.", name.trim(), age.trim());
}
