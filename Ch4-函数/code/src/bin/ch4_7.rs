// 位于ch4_7.rs作用域
fn plus_one_i32 (param1: i32) -> i32 {
    println!("ch4_7.rs scope");
    param1 + 1
}

fn main() {
    // 位于ch4_7.rs-main作用域
    fn plus_one_i32 (param1: &i32) -> i32 {
        println!("ch4_7.rs-main scope");
        param1 + 1
    }

    // 用位于ch4_7.rs-main作用域的plus_one_i32
    println!("main: {}", plus_one_i32(&1));

    // 调用位于ch4_7.rs作用域的plus_one_i32
    println!("main: {}", self::plus_one_i32(0));
}