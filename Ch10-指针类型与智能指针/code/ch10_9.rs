use std::rc::Rc;
enum List{
    Pair(i32, Rc<List>),
    Nil,
}
use crate::List::{Pair, Nil};
fn main() {
    let a = Rc::new(Pair(10, Rc::new(Pair(100, Rc::new(Nil)))));
    let b = Pair(1, Rc::clone(&a));
    let c = Pair(2, Rc::clone(&a));
    println!("strong reference count = {:?}", Rc::strong_count(&a));
}
