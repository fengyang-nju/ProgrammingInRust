	fn main() {
	    // 创建一个包含整数的向量
	    let mut numbers = vec![1, 2, 3, 4, 5];
	
	    // 使用[]操作符访问向量的元素
	    let first_number = numbers[0]; //first_number为1
	    // 下面这行代码会导致越界panic，因为索引超出了向量的范围
	    // let sixth_number = numbers[5]; 
	    // 使用get方法访问向量的元素
	    let sixth_number_option = numbers.get(5); //sixth_number_option为None
	}
    