use std::cell::Cell;
struct Example{
    a: i32,
    b: Cell<i32>
}
fn main() {
    let example = Example{ a: 0, b: Cell::new(1)}; // 创建Example的一个不可变实例
    println!("example.b = {}", example.b.replace(10)); // replace 修改字段b指向的值，并返回原值 输出：example.b = 1
    println!("example.b = {}", example.b.get()); // getter 获取字段b指向的值 输出：example.b = 10
    let temp = Cell::new(100); // 创建一个新的Cell对象temp，存放值100
    example.b.swap(&temp); // swap 交换字段b和temp的值
    println!("example.b = {}", example.b.get()); // getter 获取字段b指向的值 输出：example.b = 100
    println!("temp = {}", temp.get()); // getter 获取temp指向的值 输出：temp = 10
}