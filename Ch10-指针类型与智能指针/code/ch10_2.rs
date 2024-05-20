fn main(){

    // as 类型转换获取原生指针
    let s = String::from("Hello, Rust");
    let str_1: &String = &s; // 注意 这里str_1的类型是&String
    let ptr_1: *const String = str_1 as *const String;
    // 上面两行也可以写成：
    // let ptr_1 = &s as *const String; 
    // 同理，若第一行s声明为mutable，那么可以let ptr_1 = &mut s as *mut String; 
    // 来获取可变原生指针
    unsafe{
        println!("{}", *ptr_1); // 解引用原生指针需要用unsafe封装
    }

    println!("----------------------");


    // as_ptr()获取原生指针
    let ptr_2: *const u8 = str_1.as_ptr(); // str_1 为&String类型
    unsafe{
        println!("{}", *ptr_2);
    }

    println!("----------------------");

    let ptr_3: *const u8 = "Hello".as_ptr(); // "Hello"为&str类型
    unsafe{
        println!("{}", *ptr_3);
    }

    println!("----------------------");

    let ptr_4: *const u8 = s.as_ptr(); // s为String类型
    unsafe{
        println!("{}", *ptr_4);
    }
}
