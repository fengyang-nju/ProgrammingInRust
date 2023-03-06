fn main() {
    let incr = |x|{
        x + 1
    };

    // 显示标注参数类型和返回值类型
    let incr_same = |x: i32| -> i32 {
        x + 1
    };

    assert_eq!(incr(1), 2);
    assert_eq!(incr_same(1), 2);
    // 闭包的类型在推导确定后不能再改变
    // assert_eq!(incr(1.0f64), 2.0f64);
}