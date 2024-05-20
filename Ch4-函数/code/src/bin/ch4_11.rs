// 取参数s串最后一位的u8编码
fn tail(s: &String) -> u8 {
    if !s.is_empty() {
        s.as_bytes()[s.len() - 1]
    }
    else {
        0
    }
}

fn main() {
    let s = String::from("ProgrammingInRust");
    assert_eq!(tail(&s), 't' as u8);
    // 引用传递参数时，调用者能够保持参数变量的所有权
    assert_eq!(tail(&s), 't' as u8);
}