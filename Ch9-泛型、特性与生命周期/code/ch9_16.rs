use std::io;

fn main() {
    // 预定义的字符串切片
    let str_1 = "Hello, world"; // str_1--------------+
    // 读取标准输入                                    |
    println!("Please enter a string:"); //            |
    let mut input = String::new(); // input--------+  |
    io::stdin()                           //       |  |
        .read_line(&mut input)             //       |  |
        .expect("Failed to read line");   //       |  |
    let r; // r---------------------------------+  |  |
    if input.len() > 0{ // 输入长度合法          |  |  |
        // 移除字符串末尾的换行符                 |  |  |
        let str_2 = input.trim(); // str_----+  |  |  |
        // 获取最长的字符串切片                |  |  |  |
        r = longest(str_1, str_2); //         |  |  |  |
        println!("The longest: {}", r);//    |  |  |  |
    } // <-----------------------------------+--+  |  |
// r和str_2失效                                   |  |
}// <----------------------------------------------+--+   

// 无歧义多参数
fn longest<'a: 'c, 'b: 'c, 'c>(x: &'a str, y: &'b str) -> &'c str{
    if x.len() > y.len() { x } else { y }
}

// 有歧义多参数
// fn longest<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str{
//     if x.len() > y.len() { x } else { y }
// }

// 编译错误多参数
// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str{
//     if x.len() > y.len() { x } else { y }
// }
// error: lifetime may not live long enough