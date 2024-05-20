	fn main() {
	    let mut s = String::from("hello");
	    s.push_str(", world!"); // 在String上追加字符串
	    println!("{}", s);
	
	    let s_slice: &str = &s[0..5]; // 从String创建字符串切片
	    println!("{}", s_slice);
	}
    