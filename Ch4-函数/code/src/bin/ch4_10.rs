// 给参数s1拼接"Rust"并输出
fn append(mut s1: String) {
    s1.extend(['R', 'u', 's', 't'].iter());
    println!("{}", s1);
}

fn main() {
    let s1 = String::from("ProgrammingIn");
    append(s1);
}