// 给参数s1拼接"Rust"并输出
fn append_rust(s1: &mut String) {
    s1.extend(['R', 'u', 's', 't'].iter());
    println!("{}", s1);
}

fn main() {
    let mut s1 = String::from("ProgrammingIn");
    append_rust(&mut s1);
    // 对可变引用参数的修改具有副作用
    assert_eq!(s1, String::from("ProgrammingInRust"));
}