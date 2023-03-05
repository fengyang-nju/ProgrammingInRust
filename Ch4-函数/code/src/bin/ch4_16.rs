// 计算一个二维向量的x轴投影
// 忽略y分量
fn projection_x((x, _): &(i32, i32)) -> (i32, i32) {
    return (*x, 0)
}

#[derive(PartialEq, Debug)]
struct Vec2I32{
    x: i32,
    y: i32
}

fn projection_x_vec_2_i32(Vec2I32{x, ..}: &Vec2I32) -> Vec2I32 {
    Vec2I32 { x: *x, y: 0 }
}

fn main() {
    let v = (3, 4);
    assert_eq!(projection_x(&v), (3, 0));

    let v = Vec2I32{x: 3, y: 4};
    assert_eq!(projection_x_vec_2_i32(&v), Vec2I32 { x: 3, y: 0 });
}