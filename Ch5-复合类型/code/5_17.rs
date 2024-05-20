	fn slice_sum(slice: &[i32]) -> i32 {
	    slice.iter().sum()
	}
	
	let data = [1, 2, 3, 4, 5];
	let sum = slice_sum(&data[1..4]);
	
	println!("Sum: {}", sum); // 输出: Sum: 9
    