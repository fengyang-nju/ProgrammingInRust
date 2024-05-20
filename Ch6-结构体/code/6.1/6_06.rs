struct User {
    name: String,
    email: String,
    level: u8,
    active: bool,
}

fn init_user(name: String, email: String) -> User {
    User {
        name: name,
        email: email,
        level: 0,
        active: true,
    }
}