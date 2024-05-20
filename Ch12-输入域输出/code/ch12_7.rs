fn make_string(a: &str, b: &str) -> String {
    format!("{b} {a}")
    }

fn main(){
    println!("{}", make_string("world!",  "hello"));
}