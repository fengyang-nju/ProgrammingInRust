const C: i32 = 1;

fn main() {
    let x = 1;
    const X: i32 = C;

    // const Y: i32 = x; // Not ok.
    // const Z: i32; // Not ok.
    // let C = 1; // Not ok.
    // C = 1; // Not ok.

    assert_eq!(C + X + x, 3);
}
