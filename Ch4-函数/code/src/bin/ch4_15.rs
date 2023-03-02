// 计算一个二维向量的模
// 模式匹配解构元组
fn module_i32((x, y): &(i32, i32)) -> f64 {
    return f64::sqrt((x.pow(2) + y.pow(2)) as f64)
}

struct Vec2I32{
    x: i32,
    y: i32
}

// 模式匹配解构结构体
fn module_vec_2_i32(Vec2I32{x, y}: &Vec2I32) -> f64 {
    return f64::sqrt((x.pow(2) + y.pow(2)) as f64)
}

fn main() {
    let v = (3, 4);
    assert_eq!(module_i32(&v), 5f64);

    let v = Vec2I32{x: 3, y: 4};
    assert_eq!(module_vec_2_i32(&v), 5f64);
}