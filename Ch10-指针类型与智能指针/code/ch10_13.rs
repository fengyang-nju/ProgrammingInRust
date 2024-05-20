use std::cell::RefCell;
struct Example{
    a: i32,
    b: RefCell<String>
}
fn main(){
    let example = Example{ a: 0, b: RefCell::new(String::from("hello, world"))}; // 创建Example的一个不可变实例
    let immutable_ref = example.b.borrow(); // borrow() 方法获取RefCell的不可变引用
    println!("example.b = {:?}", immutable_ref); // 输出结果为：example.b = "hello, world"

    // 下述注释代码将触发panic：已有一个不可变借用存在，不可以再获取一个可变借用
    // let mutable_ref = example.b.borrow_mut();
}