fn main() {
    let x: u8 = 0;
    assert_eq!(x, 0u8);

    assert_eq!(0xffff, u16::MAX);
    assert_eq!(0x0000, u16::MIN);

    assert_eq!(0x7fffffff, i32::MAX);
    assert_eq!(0x80000000u32 as i32, i32::MIN);
}
