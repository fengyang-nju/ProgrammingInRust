	fn main() {
	    let mut nums = vec![1, 2, 3, 4, 5];
	    let nums_iter = nums.iter();
	
	    nums.push(6); // Error: cannot borrow `nums` as mutable because it is also borrowed as immutable
	    for num in nums_iter {
	        println!("{}", num);
	    }
	}
    