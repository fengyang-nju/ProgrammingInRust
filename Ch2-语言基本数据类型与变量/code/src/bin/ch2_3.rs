fn main() {
    let x: char = 'x';
    assert_eq!(x as u32, 0x78u32);

    // Use ASCII hex code to define a char
    assert_eq!(x, '\x78');
    // Use Unicode hex code to define a char
    assert_eq!(x, '\u{78}');

    // will print: '\'' is '
    let single_quotation_mark: char = '\'';
    println!("\'\\\'\' is {}", single_quotation_mark);

    // will print: '\' is \
    let backslash: char = '\\';
    println!("\'\\\' is {}", backslash);

    assert_eq!(0x10ffffu32, char::MAX as u32);
}
