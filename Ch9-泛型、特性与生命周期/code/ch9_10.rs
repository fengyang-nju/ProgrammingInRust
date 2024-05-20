trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

fn compare_area_static<T: Shape, U: Shape>(shape1: &T, shape2: &U) -> bool{
    return shape1.area() > shape2.area();
}

fn main() {
    let rect = Rectangle { width: 5.0, height: 5.0 };
    let circle = Circle { radius: 3.0 };
    if compare_area_static(&rect, &circle){
        println!("The rectangle is larger!");
    }
    else { println!("The circle is larger!"); }
}
// 输出：The circle is larger
