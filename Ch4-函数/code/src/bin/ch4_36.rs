use std::{cell::RefCell, rc::Rc};

fn add_triple(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

fn curried_add_triple(a: i32) -> Box<dyn Fn(i32) -> Box<dyn Fn(i32) -> i32>> {
    Box::new(move |b| -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |c| {
            add_triple(a, b, c)
        })
    })
}

fn main() {
    assert_eq!(add_triple(1, 2, 3), 6);
    assert_eq!(curried_add_triple(1)(2)(3), 6);

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                List::Cons(_, item) => Some(item),
                List::Nil => None,
            }
        }
    }

    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    println!("a next item = {:?}", a.tail());
    
}