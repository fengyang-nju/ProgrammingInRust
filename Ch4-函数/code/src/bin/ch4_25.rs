fn main() {
    let mut i = 10;

    let mut incr_i = || {
        i += 1;
        i
    };

    assert_eq!(incr_i(), 11);
    assert_eq!(i, 11);

    let mut incr_i_move = move || {
        i += 1;
        i
    };

    assert_eq!(incr_i_move(), 12);
    assert_eq!(i, 11);
}