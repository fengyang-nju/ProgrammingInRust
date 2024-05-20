use std::rc::Rc;
fn main() {
    let a = Rc::new("Hello, world");
    let a_1 = a.clone();
    let a_2 = Rc::clone(&a);
    println!("strong reference count = {:?}", Rc::strong_count(&a));
}