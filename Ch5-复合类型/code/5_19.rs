	fn main() {
	    let mut data = [10, 20, 30, 40, 50];
	
	    // 创建一个可变切片，引用数组的中间三个元素
	    let slice_mut = &mut data[1..4];
	
	    // 修改可变切片中的第一个元素
	    slice_mut[0] = 25; // data现在是 [10, 25, 30, 40, 50]
	
	    println!("Updated data: {:?}", data);
	}
    