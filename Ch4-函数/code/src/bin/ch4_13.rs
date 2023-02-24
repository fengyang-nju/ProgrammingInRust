// 返回和最大的数组引用
fn bigger_sum<'a>(v1: &'a [i32], v2: &'a [i32]) -> &'a [i32] {
    if v1.iter().sum::<i32>() > v2.iter().sum() {
        v1
    }
    else {
        v2
    }
}

fn main() {
    let v1 = [1, 2, 3];
    let v;
    // scope
    {
        let v2 = [1, 2];
        v = bigger_sum(&v1, &v2);
        
        println!("{:?}", v);
    }
    // v2只在scope内有效
    // 尽管运行时的返回值是&v1，但是借用检查器将v的生命
    // 周期取为v1和v2的交集，使得v也只在scope中有效
    // 下面的这行代码将导致编译错误
    // println!("{:?}", v);
}