use std::ops::Deref;
// 仿照Box<T>自定义一个智能指针
struct MySmartPointer<T> (T);

impl<T> MySmartPointer<T> {
    fn new(p: T) -> MySmartPointer<T> {
        MySmartPointer(p)
    }
}
// 实现Deref trait
impl<T> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 3;
    let y = MySmartPointer::new(x);
    assert_eq!(3, x);
    assert_eq!(3, *y); // 可以对MySmartPointer使用解引用操作符
}

//5 6 7 都是deref的内容