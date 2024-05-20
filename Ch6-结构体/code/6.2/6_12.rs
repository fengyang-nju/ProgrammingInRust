// error[E0106]: missing lifetime specifier
// struct User {
//     name: &str,
//     email: &str,
//     level: u8,
//     active: bool,
// }

struct User<'a> {
    name: &'a str,
    email: &'a str,
    level: u8,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "example@exa.com",
        name: "someone",
        active: true,
        level: 2,
    };
}