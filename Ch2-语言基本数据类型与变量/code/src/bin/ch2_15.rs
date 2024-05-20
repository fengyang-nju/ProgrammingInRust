fn main() {
    // cast x: i32 as y: i8
    let x = 0i32;
    let y = x as i8;
    assert_eq!(y, 0);

    // cast x: i32 as y: u8, will be cut short
    let x = 0x7fffff61i32;
    let y = x as u8;
    assert_eq!(y, 0x61u8);

    // cast y: u8 as char; only u8 can be cast as char
    let u = y as char;
    assert_eq!('a', u);

    // cast bool as integer;
    assert_eq!(true as i32, 0x1);
    assert_eq!(false as i32, 0x0);
}
