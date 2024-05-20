use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut count = 0;

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");

        if line == "quit" {
            break;
        }

        count += 1;
    }

    println!("You entered {} lines.", count);
}