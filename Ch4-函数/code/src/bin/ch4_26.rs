fn main() {
    let s1 = "ProgrammingInRust".to_string();

    let log = || println!("{}", s1);

    log();
    log();

    let log_move = move || println!("{}", s1);

    log_move();
    log_move();

    // error: borrow of moved value "s1"
    // println!("{:?}", s1);

    // error: borrow of moved value "s2";
    // s2.as_str();
}