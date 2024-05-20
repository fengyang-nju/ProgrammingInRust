enum Res {
    Success(u8),
    Failure(String),
}

impl Res {
    // ...
    fn unwrap(self) -> u8 {
        match self {
            Res::Success(level) => level,
            Res::Failure(msg) => panic!("{}", msg),
        }
    }
}
