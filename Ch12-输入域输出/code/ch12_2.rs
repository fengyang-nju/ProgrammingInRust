use std::env;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    println!("Number of arguments: {}\n", args.len());
    for (i, x) in args.iter().enumerate(){
        println!("Argument {}: {}\n", i, x);
    }
}