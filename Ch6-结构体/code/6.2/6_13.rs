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

    let User { name, level, email, active } = user1;
    let l = user1.level;
    // let n = user1.name;  error[E0382]: use of moved value: `user1.name`

}