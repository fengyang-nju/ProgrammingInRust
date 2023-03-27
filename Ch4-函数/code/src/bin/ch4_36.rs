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
}