//表达式作返回值的示例
fn plus_one_i32 (param1: i32) -> i32 {
    param1 + 1
}

fn main() {
    println!("{}", plus_one_i32(0));
}