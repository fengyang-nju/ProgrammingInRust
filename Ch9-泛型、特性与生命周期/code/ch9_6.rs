struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32>{
    fn distance(&self, other: &Point<f64>) -> f64 {
        let x_diff = self.x as f64 - other.x;
        let y_diff = self.y as f64 - other.y;
        ((x_diff * x_diff) + (y_diff * y_diff)).sqrt()
    }
}

// impl Point<i16>{
//     fn distance(&self, other: &Point<f64>) -> f64 {
//         与Point<i32>::distance的实现相同
//     }
// }

fn main() {
    let integer_point = Point { x: 5, y: 7 };
    let float_point = Point { x: 1.0, y: 4.0 };
    let distance = integer_point.distance(&float_point);
    // 输出：distance = 5
    println!("Distance: {}", distance);
}
