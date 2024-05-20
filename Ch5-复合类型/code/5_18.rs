		let mut data = [1, 2, 3, 4, 5];
	
	// 不可变切片
	let slice_immutable = &data[1..3];
	
	// 可变切片
	let slice_mutable = &mut data[2..4];
	slice_mutable[0] = 6;
