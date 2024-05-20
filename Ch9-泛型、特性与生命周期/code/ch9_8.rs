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

fn main(){
    let rect = Rectangle{ width: 10.0, height: 5.0 };
    let circle = Circle { radius: 5.0 };
    println!("Area of rect is {}", rect.area());
    println!("Area of circle is {}", circle.area());
}