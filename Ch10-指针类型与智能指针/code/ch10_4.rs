#[derive(Debug)]
struct Point(i32, i32);
fn main() {
    let a = Box::new(Point(1,2));
    println!("a = {:?}", *a);
}

