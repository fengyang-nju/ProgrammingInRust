fn hello(s: &str) {
    println!("Hello, {}", s);
}

fn main(){
    let b = Box::new(String::from("world"));
    hello(&b);
}