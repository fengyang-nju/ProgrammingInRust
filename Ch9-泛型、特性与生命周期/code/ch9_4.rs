struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    // 横、纵坐标为整型
    let integer_point = Point { x: 5, y: 7 };
    // 横、纵坐标为浮点型
    let float_point = Point { x: 1.0, y: 4.0 };
}
