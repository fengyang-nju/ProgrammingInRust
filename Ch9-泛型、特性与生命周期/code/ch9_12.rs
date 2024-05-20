trait Shape {
    fn area(&self) -> f64;
    // 静态方法，提供该形状类型的描述
    fn description() -> String where Self: Sized;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }

    fn description() -> String {
        "All points equidistant from a center point".to_string()
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn description() -> String {
        "A shape with four sides and right angles".to_string()
    }
}

fn compare_area_dyn(shape1: &dyn Shape, shape2: &dyn Shape) -> bool{
    return shape1.area() > shape2.area();
}

fn main() {
    let rect = Rectangle { width: 5.0, height: 5.0 };
    let circle = Circle { radius: 3.0 };
    if compare_area_dyn(&rect, &circle){
        println!("The rectangle is larger!");
    }
    else { println!("The circle is larger!"); }
}