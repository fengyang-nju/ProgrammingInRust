fn init_user(name: String, email: String) -> User {
    User {
        name,
        email,
        level: 0,
        active: true,
    }
}