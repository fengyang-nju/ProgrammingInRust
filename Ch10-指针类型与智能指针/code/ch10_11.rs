use std::cell::Cell;
struct Example{
    a: i32,
    b: Cell<i32>
}
fn main() {
    let example = Example{ a: 0, b: Cell::new(1)}; // 创建Example的一个不可变实例
    println!("example.b = {}", example.b.get()); // getter 获取字段b指向的值 输出：example.b = 1
    example.b.set(10); // setter 修改字段b指向的值 
    println!("example.b = {}", example.b.get()); // getter 获取字段b指向的值 输出：example.b = 10
}