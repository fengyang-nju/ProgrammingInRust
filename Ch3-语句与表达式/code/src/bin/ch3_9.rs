fn main() {
    let second = {
        let day = 18;
        let hour = 14;
        (day * 24 + hour) * 60 * 60
    };

    assert_eq!(second, (18 * 24 + 14) * 60 * 60);
}
