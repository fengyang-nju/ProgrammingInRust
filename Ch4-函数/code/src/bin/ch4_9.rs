// 取i32类型参数x的个位数字
fn tail(x: i32) -> i32 {
    x % 10
}

fn main() {
    let one = 1;
    assert_eq!(tail(one), 1);
    // 这里，变量one在调用tail(one)后可以被再次使用。
    assert_eq!(tail(one), 1);
}