		let range = 0..10;
	
	// 每次迭代跳过2个元素
	for num in range.step_by(2) {
	    println!("{}", num); // 将打印 0, 2, 4, 6, 8
	}
