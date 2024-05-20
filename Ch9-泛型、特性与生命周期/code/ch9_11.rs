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

fn print_area(shapes: Vec<&dyn Shape>) {
    for shape in shapes {
        println!("Shape area: {}", shape.area());
    }
}

fn main() {
    let circle = Circle { radius: 2.0 };
    let rectangle = Rectangle { width: 3.0, height: 4.0 };

    let shapes: Vec<&dyn Shape> = vec![&circle, &rectangle];
    print_area(shapes);
}
