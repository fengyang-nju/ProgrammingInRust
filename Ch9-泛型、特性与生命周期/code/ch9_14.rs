// 函数返回悬垂引用

/*fn main() {
    let reference = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
// 报错提示：this function's return type contains a borrowed value, 
// but there is no value for it to be borrowed from 
// help: consider using the `'static` lifetime
//  |
//5 | fn dangle() -> &'static String {
//  |                 +++++++
*/

// 报错提示：this function's return type contains a borrowed value, 
// but there is no value for it to be borrowed from 

// 数据竞争示例

/*fn main() {
    let mut numbers = vec![1, 2, 3]; // 创建一个Vec
    let first = &numbers[0]; // 获取第一个元素的不可变引用
    // 尝试向Vec中添加一个新元素
    numbers.push(4); // 错误：不能在有不可变引用的同时进行可变借用
    //error[E0502]: cannot borrow `numbers` as mutable because it is also borrowed as immutable
    println!("The first number is: {}", first); // 使用不可变引用
}
*/

// 修改后不存在数据竞争的实例
/*fn main() {
    let mut numbers = vec![1, 2, 3]; // 创建一个Vec
    let first = &numbers[0]; // 获取第一个元素的不可变引用
    println!("The first number is: {}", first); // 使用不可变引用
    // 尝试向Vec中添加一个新元素
    numbers.push(4); // 这次编译器不再报错
}
*/

// 非词法作用域生命周期
fn main() {
    let mut numbers = vec![1, 2, 3];
    let first = &numbers[0]; //  <--------------------------------+  
    println!("The first number is: {}", first); // first的生命周期 |
    // <----------------------------------------------------------+  
    numbers.push(4); // 这次编译器不再报错，因为first的生命周期已经结束
}
