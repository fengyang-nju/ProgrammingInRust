struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 7 };
    let float_point = Point { x: 1.0, y: 4.0 };
    // 需要将整型转为f64类型，以进行后续浮点数的运算
    // 同时，float_point的横纵坐标也被推导为f64类型
    let x_diff = integer_point.x as f64 - float_point.x;
    let y_diff = integer_point.y as f64 - float_point.y;
    let distance = (x_diff * x_diff + y_diff * y_diff).sqrt();
    // 输出：distance = 5
    println!("distance = {}", distance);
}