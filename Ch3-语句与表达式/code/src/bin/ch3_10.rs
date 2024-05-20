const ZERO: i32 = 0;

// no warning on path statements with no effect
#[allow(path_statements)]
fn main() {
    let zero = ZERO;

    ZERO;
    zero;
    0;
}
