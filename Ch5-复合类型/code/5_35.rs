	fn main() {
	    let message = "Hello, 世界!"; // 包含ASCII和非ASCII字符
	    // 字符迭代
	    for ch in message.chars() {
	        println!("{}", ch);
	    }
	    // 字节迭代
	    println!("Bytes:");
	    for byte in message.bytes() {
	        println!("{}", byte);
	    }
	}
    