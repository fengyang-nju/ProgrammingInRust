fn main() {
    let mut s = "ProgrammingInRust".to_string();

    let mut insert_rust_str = || {
        s.insert_str(0, "Read ");
    };

    insert_rust_str();
    assert_eq!(
        s, 
        "Read ProgrammingInRust".to_string()
    );
}