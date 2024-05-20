const ZERO: i32 = 0;
fn main() {
    let one = 1;
    assert_eq!(1, ZERO + one);
    assert_eq!(1, one - 0);
    assert_eq!(0, 1 * 0);
    assert_eq!(0, 0 / 1);
    assert_eq!(0, 0 % 1);
    assert_eq!(2, 0 + 1 + 1);
    assert_eq!(-1, -one);
}
