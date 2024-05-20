#[derive(Debug)]
struct S {
    x: i32,
    y: f32
}

fn main() {
    let a = S{x:34, y:23.3};
    let b = a;
    // println!("{:?}", a); error[E0382]: borrow of moved value: `a`
}