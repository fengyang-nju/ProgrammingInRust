	fn main() {
	    let mut text = String::from("Hello, Rust programmers!");
	    println!("Original text: {}", text);
	
	    // 添加内容
	    text.push_str(" Welcome to the text editor.");
	    println!("After addition: {}", text);
	
	    // 删除内容
	    text.pop(); // 移除最后一个字符
	    println!("After deletion: {}",text);
	
	    // 替换内容
	    text = text.replace("text editor", "Rust workshop");
	    println!("After replacement: {}", text);
	
	    // 拼接字符串
	    let additional_info = " Let's start coding.";
	    text = format!("{}{}", text, additional_info);
	    println!("After concatenation: {}", text);
	
	    // 清空字符串
	    text.clear(); // 清空文本
	    println!("After clearing: '{}'", text);
	}
    