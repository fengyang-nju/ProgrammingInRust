fn main() {
    let x: i32 = 1;
    let y: i8 = 1;
    let f: f32 = 4.0;

    assert_eq!(x + y as i32, 2);
    assert_eq!(y as f32 + f, 5.0);
}
