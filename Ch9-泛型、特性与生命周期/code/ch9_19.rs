fn main() {
    let reference = dangle();
}

fn dangle() -> &'static String {
    let s = String::from("hello");
    &s
}
// error[E0515]: cannot return reference to local variable `s`
//  --> ch9_19.rs:7:5
//   |
// 7 |     &s
//   |     ^^ returns a reference to data owned by the current function

// fn main() {
//     let s = static_str();
//     println!("{}", s);
// }

// fn static_str() -> &'static str {
//     return "Hello, world!";
// }
