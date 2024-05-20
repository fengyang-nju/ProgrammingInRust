struct User {
    name: String,
    email: String,
    level: u8,
    active: bool,
}

impl User {
    fn new(name: String, email: String) -> Self {
        User {
            name,
            email,
            level: 0,
            active: false,
        }
    }

    fn login(&mut self) {
        self.active = true;
    }

    fn logout(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut user1 = User::new(String::from("example@exa.com"), String::from("someone"));
    user1.login();
}