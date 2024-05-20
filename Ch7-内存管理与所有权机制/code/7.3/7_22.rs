fn main() {
    let n = 123;
    let r = &n;
    assert!(*r == 123);
}