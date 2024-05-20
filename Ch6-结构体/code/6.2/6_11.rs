#[derive(Debug)]
struct User {
    name: String,
    email: String,
    level: u8,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("example@exa.com"),
        name: String::from("someone"),
        active: false,
        level: 2,
    };

    // println!("{}", user1);   error[E0277]: `User` doesn't implement `std::fmt::Display`
    println!("{:?}", user1);
}
