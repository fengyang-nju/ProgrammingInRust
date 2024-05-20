struct Point(i32, i32);

fn main() {
    let p = Point(32, 12);
    println!("x = {}, y = {}.", p.0, p.1);
}