fn append(mut str: String) -> String {
    str.push_str(", world");
    str
}
fn main() {
    let s = String::from("hello");
    let ss = append(s);
    println!("{}", ss);
}