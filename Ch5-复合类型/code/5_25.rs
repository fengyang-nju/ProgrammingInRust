	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);  //创建一个Blue-10的键值对
	scores.insert(String::from("Yellow"), 50); // 创建一个Yellow-50的键值对
	if let Some(score) = scores.get("Blue") { //判断scores中有Blue这个键
	    println!("Blue's score: {}", score); //get方法返回Some(score)，并通过if解包Some
	}
