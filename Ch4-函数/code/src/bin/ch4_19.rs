use std::ffi::{c_int};

#[link(name = "sum_pos_int")]
extern {
    fn sum_pos_int(x1: c_int, ...) -> c_int;
}

fn main() {
    let x1 = 1;
    let x2 = 2;
    let x3 = 0;
    unsafe {
        assert_eq!(sum_pos_int(x1, x2, x3), 3);
    }
}