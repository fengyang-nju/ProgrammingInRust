static S: i32 = 1;

fn main() {
    let x = 2;
    static mut Y: i32 = S;

    unsafe { Y = x };

    assert_eq!(S + unsafe { Y }, 3);
}
