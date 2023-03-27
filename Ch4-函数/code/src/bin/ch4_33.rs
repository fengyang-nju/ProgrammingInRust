//函数指针的定义和调用

use std::any::type_name;

fn plus_one_i32 (param1: i32) -> i32 {
    return param1 + 1;
}
fn main() {
    //在定义函数指针时需要标注类型
    let ptr: fn(i32) -> i32 = plus_one_i32;
    println!("{:p}", ptr);
    print_type(&ptr);

    let fake_ptr = plus_one_i32;
    print_type(&fake_ptr);

    //函数指针的调用
    assert_eq!(ptr(1), 2);
}

fn print_type<T>(_: &T) {
    println!("{}", type_name::<T>());
}