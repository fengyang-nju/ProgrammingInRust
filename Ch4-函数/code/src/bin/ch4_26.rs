fn main() {
    let s1 = "ProgrammingInRust".to_string();
    let s2 = "RustProgrammingLanguage".to_string();

    let bigger_string = || {
        s1.max(s2)
    };
    assert_eq!(
        bigger_string(), 
        "RustProgrammingLanguage".to_string()
    );

    // error: closure cannot be invoked more than once because
    // it moves the variable `s1` out of its environment.
    // assert_eq!(
    //     max_string(), 
    //     "RustProgrammingLanguage".to_string()
    // );

    // error: borrow of moved value "s1"
    // println!("{:?}", s1);

    // error: borrow of moved value "s2";
    // s2.as_str();
}
