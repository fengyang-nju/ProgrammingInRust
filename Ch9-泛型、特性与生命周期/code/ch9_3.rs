fn equal<T>(a1: T, a2: T, compare: fn(&T, &T) -> bool) -> bool {
    compare(&a1, &a2)
}

fn main() {
    // 对于浮点数，检查差的绝对值是否小于某个阈值
    // error[E0689]: can't call method `abs` on ambiguous numeric type `{float}`
    // let float_equal = equal(1.0, 1.0001, |a, b| (a - b).abs() < 0.001);
    let float_equal = equal::<f32>(1.0, 1.0001, |a, b| (a - b).abs() < 0.001);
    // 输出: 1.0 and 1.0001 are 'equal': true
    println!("1.0 and 1.0001 are equal: {}", float_equal); 
}
