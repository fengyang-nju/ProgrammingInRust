fn main() {
    let x = 1;
    let mut y = 2;
    let z = 3;

    assert_eq!(x + y - z, (x + y) - z);

    let x = y = z;
    assert_eq!(x, ());
    assert_eq!(y, 3);

    assert_eq!((y > z) == false, true);
    assert_eq!((y > z) == (z <= y), false);
}
