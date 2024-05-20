		fn calculate_stats(numbers: &[i32]) -> (f64, i32) {
	    let sum: i32 = numbers.iter().sum();
	    let avg: f64 = sum as f64 / numbers.len() as f64;
	    (avg, sum)
	}
	
	let numbers = [1, 2, 3, 4, 5];
	let (average, total) = calculate_stats(&numbers);
	
	println!("Average: {}, Total: {}", average, total);
    