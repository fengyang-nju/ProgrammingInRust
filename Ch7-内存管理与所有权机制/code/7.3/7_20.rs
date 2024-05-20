fn main() {
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    // let s3 = &s1;       error
    // let s4 = &mut s1;   error
    // s1.push('a');       error
    // let ss = s1;        error
    s2.push('a');
}
