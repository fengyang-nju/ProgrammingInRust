struct Point<T> {
    x: T,
    y: T,
}

use std::convert::Into;
impl<T> Point<T>
    where T: Into<f64> + Copy,
{
    fn distance<U>(&self, other: &Point<U>) -> f64 
        where U: Into<f64> + Copy,
    {
        let x_diff = self.x.into() - other.x.into();
        let y_diff = self.y.into() - other.y.into();
        ((x_diff * x_diff) + (y_diff * y_diff)).sqrt()
    }
}

fn main() {
    let integer_point = Point { x: 5, y: 7 };
    let float_point = Point { x: 1.0, y: 4.0 };
    let distance = integer_point.distance(&float_point);
    println!("Distance: {}", distance);
}