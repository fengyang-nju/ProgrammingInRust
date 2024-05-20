struct Book<'a> {
    title: &'a str,
}

fn main() {
    let book_title = String::from("The Rust Programming Language");
    let book = Book {
        title: book_title.as_str(),
    };
    println!("Book title: {}", book.title);
}
