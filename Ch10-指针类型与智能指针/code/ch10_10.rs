use std::rc::Rc;
fn main() {
    let a = Rc::new("Hello, world");
    let w_1 = Rc::downgrade(&a);
    let w_2 = Rc::downgrade(&a);
    println!("strong reference count = {:?}", Rc::strong_count(&a)); // 1
    println!("weak reference count = {:?}", Rc::weak_count(&a)); // 2
    
    let a_1 = w_1.upgrade(); // a_1 与 a 共享所有权
    println!("strong reference count = {:?}", Rc::strong_count(&a)); // 2
    println!("weak reference count = {:?}", Rc::weak_count(&a)); // 2
}