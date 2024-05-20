fn main() {
    let s1 = String::from("Hello, world. 你好，世界。");
    let num = count_ascii(&s1);
    println!("The number of ASCII chars in '{}' is {}.", s1, num);
}
fn count_ascii(s: &String) -> usize {
    let mut num = 0;
    for char in s.chars() {
        if char.is_ascii() {
            num += 1;
        }
    }
    num
}