fn abs_i32(x: i32) -> i32 {
    if x > 0 {
        //这里不能用表达式‘x’代替return语句
        //因为表达式无法让控制流提前返回
        return x;
    }
    -x
}

fn main() {
    println!("{}", abs_i32(1));
    println!("{}", abs_i32(-1));
}