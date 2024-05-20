	fn main() {
	    let nums = vec![1, 2, 3, 4, 5];
	
	    // 过滤出偶数
	    let evens: Vec<_> = nums.iter().filter(|&&x| x % 2 == 0).collect();
	    println!("Even numbers: {:?}", evens);
	
	    // 将每个数字乘以 2
	    let doubled: Vec<_> = nums.iter().map(|&x| x * 2).collect();
	    println!("Doubled numbers: {:?}", doubled);
	
	    // 计算所有数字的总和
	    let sum: i32 = nums.iter().fold(0, |acc, &x| acc + x);
	    println!("Sum of numbers: {}", sum);
	}
    