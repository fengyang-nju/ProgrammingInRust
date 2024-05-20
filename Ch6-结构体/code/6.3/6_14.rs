struct User {
    name: String,
    email: String,
    level: u8,
    active: bool,
}

impl User {
    fn login(&mut self) {
        self.active = true;
    }

    fn logout(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("example@exa.com"),
        name: String::from("someone"),
        active: false,
        level: 2,
    };
    user1.login();
}