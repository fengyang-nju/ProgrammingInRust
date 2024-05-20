		let numbers = [1, 2, 3, 4, 5];
	
	// 创建一个包含前两个元素的切片
	let first_two = &numbers[..2];
	
	// 创建一个包含除了前两个元素之外所有元素的切片
	let rest = &numbers[2..];
	
	// 创建一个包含中间三个元素的切片
	let middle = &numbers[1..4];
