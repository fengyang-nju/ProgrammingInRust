trait Shape {
    fn area(&self) -> f64;
}

trait Colored: Shape {
    fn color(&self) -> String;
    fn set_color(&mut self, color: String);
}

struct Circle {
    radius: f64,
    color: String,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.141592 * self.radius * self.radius
    }
}

impl Colored for Circle {
    fn color(&self) -> String {
        self.color.clone()
    }

    fn set_color(&mut self, color: String) {
        self.color = color;
    }
}

fn main() {
    let mut circle = Circle{ radius: 5.0, color: "red".to_string() };
     // 输出：Circle color: red
    println!("Circle color: {}", circle.color());
    circle.set_color("blue".to_string());
     // 输出：Circle new color: blue
    println!("Circle new color: {}", circle.color());
}
