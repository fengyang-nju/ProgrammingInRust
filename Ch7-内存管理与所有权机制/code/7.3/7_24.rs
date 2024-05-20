fn main() {
    let mut v = vec![12, 32, 6, 3, 45];
    v.sort();
    (&mut v).sort();
}