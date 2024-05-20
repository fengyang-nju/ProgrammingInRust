fn main() {
    let x = 3;

    assert_eq!(x > 1 && x > 2, true);
    assert_eq!(x > 2 || x > 3, true);
    assert_eq!(!x > 2, false);
}
