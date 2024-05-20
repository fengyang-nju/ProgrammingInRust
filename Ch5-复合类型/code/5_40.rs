	fn main() {
	    let vec = vec![1, 2, 3, 4, 5];
	
	    let iter = vec.iter(); // 创建不可变迭代器
	    let iter_mut = vec.iter_mut(); // 创建可变迭代器（此行单独存在时编译错误，因为iter_mut没有被消费）
	    let into_iter = vec.into_iter(); // 创建消费迭代器
	
	    // 使用for循环遍历iter
	    for val in iter {
	        println!("Value: {}", val);
	    }
	}
    