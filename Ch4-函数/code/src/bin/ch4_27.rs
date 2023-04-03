fn main() {
    let s = "ProgrammingInRust".to_string();

    let log_move = move || println!("{}", s);

    log_move();
    log_move();     //能够多次调用

    // error: borrow of moved value "s"
    // println!("{:?}", s);
}