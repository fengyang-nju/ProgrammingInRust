const MODERATE_SALT: u8 = 3;

fn main() {
    let mut salt = 4;

    salt = match salt {
        0 | 1 | 2 => salt + MODERATE_SALT,
        _ if salt > 2 * MODERATE_SALT => {
            println!("Too salty!");
            salt
        }
        _ => salt,
    };

    assert!(salt >= MODERATE_SALT);
}
