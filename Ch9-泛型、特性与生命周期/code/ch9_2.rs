fn equal<T>(a1: T, a2: T, compare: fn(&T, &T) -> bool) -> bool {
    compare(&a1, &a2)
}

fn main() {
    // 对于整数，直接使用相等比较
    let int_equal = equal(5, 5, |a, b| a == b);
    // 输出: 5 and 5 are equal: true
    println!("5 and 5 are equal: {}", int_equal); 

    // 对于字符串常量，直接使用相等比较
    let str_equal = equal("Rust", "Rust", |a, b| a == b);
    // 输出: 'Rust' and 'Rust' are equal: true
    println!("'Rust' and 'Rust' are equal: {}", str_equal); 
}
