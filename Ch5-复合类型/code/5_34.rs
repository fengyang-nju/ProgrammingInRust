		fn main() {
	    let mut message = String::from("Hello, ");
	    let name = "John Doe";
	    message.push_str(name); // 添加内容
	    println!("Greeting: {}", message);
	
	    // 尝试修改字符串中的部分内容（不可变引用与可变引用冲突示例）
	    let part_of_message = &message[7..]; // 创建不可变引用
	   
	    message.clear(); // 编译错误
		 println!("Extracted part: {}", part_of_message);
	  
	println!("Message after clear: '{}'", message);
	}
    