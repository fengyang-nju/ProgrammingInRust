	// 使用glob导入符号引入
	use shapes::*;
	
	fn main() {
	    let circle = Circle { radius: 5.0 };
	    let square = Square { side: 3.0 };
	    draw_circle(&circle);
	    draw_square(&square);
	}
