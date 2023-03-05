use std::ffi::c_int;

#[link(name = "add_one")]
extern {
    fn add_one(x: c_int) -> c_int;
}

fn main() {
    let x = 1;
    unsafe {
        assert_eq!(add_one(x), 2);
    }
}