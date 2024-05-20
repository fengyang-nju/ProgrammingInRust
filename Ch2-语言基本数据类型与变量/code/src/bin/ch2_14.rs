fn main() {
    assert_eq!(1, counter());
    assert_eq!(2, counter());
    assert_eq!(counter() + 1, counter());
}

fn counter() -> i32 {
    static mut COUNT: i32 = 0;

    unsafe {
        COUNT += 1;
        COUNT
    }
}
