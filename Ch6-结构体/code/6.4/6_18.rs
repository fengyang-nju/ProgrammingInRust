enum Res {
    Success(u8),
    Failure(String),
}

fn main() {
    let res = Res::Success(23);
    let res = Res::Failure(String::from("No such user."));
}