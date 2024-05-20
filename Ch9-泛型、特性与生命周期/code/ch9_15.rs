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

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() { x } else { y }
}

// 缺少生命周期标注的函数
// fn longest(x: &str, y: &str) -> &str{
//     if x.len() > y.len() { x } else { y }
// }
// error[E0106]: missing lifetime specifier