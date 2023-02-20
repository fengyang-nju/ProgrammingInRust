// 给参数s1拼接"Rust"并输出
fn append(mut s1: &mut String) {
    s1.extend(['R', 'u', 's', 't'].iter());
    println!("{}", s1);
    
}

fn main() {
    let mut s1 = String::from("ProgrammingIn");
    append(&mut s1);
    assert_eq!(s1, String::from("ProgrammingInRust"));
}