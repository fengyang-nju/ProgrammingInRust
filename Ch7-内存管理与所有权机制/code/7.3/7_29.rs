fn main() {
    let r = &sum(10);
    assert_eq!(2 * &12, 24);
}

fn sum(n: i32) -> i32 {
    (1..n).sum()
}