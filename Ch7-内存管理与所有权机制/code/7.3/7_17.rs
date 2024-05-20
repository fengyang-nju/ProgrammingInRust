fn main() {
    let s1 = String::from("Hello, world. 你好，世界。");
    let (s2, num) = count_ascii(s1);
    println!("The number of ASCII chars in '{}' is {}.", s2, num);
}

fn count_ascii(s: String) -> (String, usize) {
    let mut num = 0;
    for char in s.chars() {
        if char.is_ascii() {
            num += 1;
        }
    }
    (s, num)
}