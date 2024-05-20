	fn main() {
	    let nums = vec![1, 2, 3, 4, 5];
	    let processed: Vec<_> = nums.iter()
	                                 .map(|x| x * x)
	                                 .filter(|x| x % 2 == 0)
	                                 .collect();
	    println!("Processed numbers: {:?}", processed);
	}
    