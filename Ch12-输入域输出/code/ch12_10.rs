use std::io::{self, Write};

fn main() {
    print!("Please enter an odd number n: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let n: usize = input
        .trim()
        .parse()
        .expect("Invalid input, please enter a positive integer");

    if n % 2 == 0 {
        panic!("Input must be odd");
    }

    for i in 0..n {
        let num_stars = if i <= n / 2 {
            i * 2 + 1
        } else {
            (n - i - 1) * 2 + 1
        };
        let num_spaces = (n - num_stars) / 2;
        print!("{: <1$}", "", num_spaces);
        println!("{:*<1$}", "", num_stars);
    }
}