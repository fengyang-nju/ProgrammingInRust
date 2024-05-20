		fn main() {
	    let nums = vec![1, 2, 3, 4, 5];
	    let sum: i32 = nums.iter()
	                       .filter(|x| *x % 2 == 0)
	                       .map(|x| x * 2)
	                       .sum();
	    println!("Sum: {}", sum);
	}
    