fn main() {
    let x = 123;
    let y = 234;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert!(rx < ry);
    assert!(rrx < rry);
}