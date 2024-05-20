fn main() {
    let mut s1 = String::from("hello");
    let s2 = &s1;
    let s3 = &s1;
    // let s4 = &mut s1;   error
    // s1.push('a');       error
    // let ss = s1;        error
    println!("{}", s2);
}