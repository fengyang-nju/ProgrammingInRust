fn main(){
    let s = "Hello, Rust";
    let ptr: *const u8 = s.as_ptr();

    // offset
    unsafe{
        println!("{}", *ptr as char);
        println!("{}", *ptr.offset(1) as char);
        println!("{}", *ptr.offset(2) as char);
    }

    println!("----------------------");

    // read
    let s_1 = "Hello, Rust";
    let ptr_1: *const u8 = s_1.as_ptr();
    unsafe{
        println!("{}", ptr_1.read() as char);
    }
    
    println!("----------------------");

    // write
    let mut s_2 = "Hello, Rust";
    let ptr_2 = &mut s_2 as *mut &str;
    unsafe{
        println!("before: {}", ptr_2.read());
        ptr_2.write("Hello, world");
        println!("after: {}", ptr_2.read());
    }
}
