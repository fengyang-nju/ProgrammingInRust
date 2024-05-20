// error[E0597]: `n` does not live long enough
fn main() {
    let r;
    {
        let n = 1;
        r = &n;
    }
    println!("{}", r);
}