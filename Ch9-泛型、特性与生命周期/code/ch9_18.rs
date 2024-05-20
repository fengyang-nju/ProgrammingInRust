struct Book<'a> {
    title: &'a str,
}
impl<'a> Book<'a> {
    // 创建一个新的Book实例的方法
    fn new(title: &'a str) -> Book<'a> {
        Book { title }
    }
    // 省略生命周期参数写法
    // fn new(title: &str) -> Book { ... }
    // 获取书名的方法
    fn title(&self) -> &str {
        self.title
    }
    // 显示书名的方法
    fn show_title(&self) {
        println!("The book title is: {}", self.title());
    }
}
fn main() {
    let book_title = String::from("The Rust Programming Language");
    let book = Book::new(&book_title);
    book.show_title();
}