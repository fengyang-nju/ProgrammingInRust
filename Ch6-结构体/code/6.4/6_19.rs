enum Res {
    Success(u8),
    Failure(String),
}

impl Res {
    fn is_success(&self) -> bool {
        matches!(self, Res::Success(_))
    }

    fn is_failure(&self) -> bool {
        !self.is_success()
    }

    fn new_success(v: u8) -> Self {
        Res::Success(v)
    }

    fn new_failure(msg: String) -> Self {
        Res::Failure(msg)
    }
}

fn main() {
    let res1 = Res::new_success(12);
    let res2 = Res::new_failure(String::from("Invalid username."));
    let flag1 = res1.is_success();
    let flag2 = res2.is_failure();
    assert_eq!(flag1, true);
    assert_eq!(flag2, true);
}