fn main() {
    let s = "ProgrammingInRust".to_string();

    let log = || {
        println!("log: {}", s);
    };

    log();
    println!("main: {}", s);
}