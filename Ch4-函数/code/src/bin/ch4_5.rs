//求i32数组v的几何平均数和算数平均数
fn mean_arithmetic_geometric(v: Vec<i32>) -> (f64, f64) {
    //初始化几何平均数为0和算术平均数为1
    let mut arithmetic_mean = 0f64;
    let mut geometric_mean = 1f64;
    //对v中的每个元素求和、求积 
    for x in &v {
        arithmetic_mean += *x as f64;
        geometric_mean *= *x as f64;
    }
    //求得几何平均数和算数平均数
    arithmetic_mean /= v.len() as f64;
    geometric_mean = geometric_mean.powf(1f64/v.len() as f64);
    //以元组形式返回几何平均数和算数平均数
    (arithmetic_mean, geometric_mean)
}

fn main() {
    println!("{:?}", mean_arithmetic_geometric(vec![1, 1, 1]));
    println!("{:?}", mean_arithmetic_geometric(vec![1, 2, 3]));
}