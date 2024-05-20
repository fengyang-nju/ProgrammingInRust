trait Shape {
    // 静态方法，提供该形状类型的描述
    fn description() -> String;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn description() -> String {
        "All points equidistant from a center point".to_string()
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn description() -> String {
        "A shape with four sides and right angles".to_string()
    }
}

fn main() {
    println!("Circle: {}", Circle::description());
    println!("Rectangle: {}", Rectangle::description());
}
