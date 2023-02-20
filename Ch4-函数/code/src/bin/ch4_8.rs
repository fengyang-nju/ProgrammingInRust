fn tail(s: String) -> u8 {
    if !s.is_empty() {
        s.as_bytes()[s.len() - 1]
    }
    else {
        0
    }
}

fn main() {
    let s = String::from("ProgrammingInRust");
    assert_eq!(tail(s), 't' as u8);
    // tail(s)获得了s的所有权，
    // tail(s)调用结束后将不能再使用s。
}