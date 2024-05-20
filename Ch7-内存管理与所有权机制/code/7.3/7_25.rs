fn main() {
    let x = 12;
    let y = 23;
    let mut r = &x;
    let b = true;
    if b {
        r = &y;
    }
}