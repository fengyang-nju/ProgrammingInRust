fn main() {
    let x = 1;
    let mut i = 10;

    let mut incr = |x| //--+
        i += x;		 //  |
//                       |	被捕获变量i的可变借用的有效期
    //error: cannot assign to i because it is borrowed
    //i = 1;    		 |
				//           |
    incr(x);	//----------+
    assert_eq!(i, 11);

    i = 1; // ok
    assert_eq!(i, 1);
}