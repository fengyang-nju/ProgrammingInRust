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

    let user_level = user1.level;
    let (user_name, user_active) = (user1.name, user1.active);
}