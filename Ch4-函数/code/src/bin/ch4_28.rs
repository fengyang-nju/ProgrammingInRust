fn main() {
    let x = 1;
    let mut i = 10;

    let mut incr_move = move |x| {
        i += x;
        return i;
    };

    assert_eq!(incr_move(x), 11);
    assert_eq!(i, 10);  //i的值并未改变，说明捕获时发生了浅拷贝

    incr_move(x); //可以重复调用
}