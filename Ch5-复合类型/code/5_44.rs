	fn main() {
	    let nums = vec![1, 2, 3, 4, 5];
	    let mut sum = 0;
	    for num in nums {
	        if num % 2 == 0 {
	            sum += num * 2;
	        }
	    }
	    println!("Sum: {}", sum);
	}
    