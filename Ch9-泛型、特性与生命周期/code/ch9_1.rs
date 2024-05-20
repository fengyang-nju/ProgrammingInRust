fn main() {
    // 整数向量
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("Numbers: {:?}", numbers); // 输出: Numbers: [1, 2, 3]

    // 字符串向量
    let mut names: Vec<String> = Vec::new();
    names.push("Alice".to_string());
    names.push("Bob".to_string());
    names.push("Carol".to_string());
    println!("Names: {:?}", names); // 输出: Names: ["Alice", "Bob", "Carol"]
}