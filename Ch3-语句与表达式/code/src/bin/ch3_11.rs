const MODERATE_SALT: u8 = 3;

fn main() {
    let mut salt = 0;

    salt = if salt < MODERATE_SALT {
        salt + MODERATE_SALT
    } else {
        salt
    };

    assert!(salt >= MODERATE_SALT);
}
