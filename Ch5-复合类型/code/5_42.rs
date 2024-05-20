	fn main() {
	    let nums = vec![1, 2, 3, 4, 5];
	    let mut temp = Vec::new();
	    for num in &nums {
	        temp.push(num * num); // 先计算平方
	    }
	    let mut processed = Vec::new();
	    for num in temp {
	        if num % 2 == 0 {
	            processed.push(num); // 再过滤偶数
	        }
	    }
	    println!("Processed numbers: {:?}", processed);
	}
    