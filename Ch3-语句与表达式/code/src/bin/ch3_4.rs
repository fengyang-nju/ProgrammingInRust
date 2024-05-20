fn main() {
    let x = 10;
    let y = 12;

    assert_eq!(x == y, false);
    assert_eq!(x != y, true);
    assert_eq!(x < y, true);
    assert_eq!(x <= y, true);
    assert_eq!(x > y, false);
    assert_eq!(x >= y, false);
}
